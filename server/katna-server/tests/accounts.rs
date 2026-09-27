// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna accounts end to end against a real PostgreSQL. Set
//! `KATNA_SERVER_TEST_DATABASE_URL` (CI's `server` job does) and run with
//! `--include-ignored`.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use katna_server::db::Db;
use katna_server::mailer::{Mailer, Purpose, SentCode};
use katna_server::{AppState, Config, router};
use serde_json::{Value, json};
use tower::ServiceExt;

const NEEDS_DB: &str = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL";

struct App {
    router: Router,
    outbox: Arc<Mutex<Vec<SentCode>>>,
}

impl App {
    async fn new() -> Self {
        let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
        let db = Db::connect(&url).unwrap();
        db.migrate().await.unwrap();
        let config = Config {
            database_url: url,
            installs_per_hour: 1000,
            ..Config::default()
        };
        let outbox = Arc::default();
        let router = router(AppState::with_mailer(
            db,
            config,
            Mailer::Memory(Arc::clone(&outbox)),
        ));
        Self { router, outbox }
    }

    async fn call(
        &self,
        method: &str,
        uri: &str,
        token: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let builder = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));
        let request = match body {
            Some(body) => builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => builder.body(Body::empty()),
        }
        .unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    async fn install(&self) -> String {
        let request = Request::post("/api/v1/installs")
            .body(Body::empty())
            .unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        body["token"].as_str().unwrap().to_owned()
    }

    fn code(&self, to: &str, purpose: Purpose) -> String {
        self.outbox
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|sent| sent.to == to && sent.purpose == purpose)
            .map(|sent| sent.code.clone())
            .expect("a code was mailed")
    }

    /// Whether `token` may use a server feature (creating a tracking ID).
    async fn feature(&self, token: &str) -> (StatusCode, Value) {
        self.call("POST", "/api/v1/tracks", token, Some(json!({ "count": 1 })))
            .await
    }
}

fn new_email() -> String {
    format!("Someone.{}@Example.com", katna_server::ids::new_id())
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn sign_up_confirm_and_use_features() {
    let app = App::new().await;
    let laptop = app.install().await;

    // Not signed in: features are refused.
    let (status, body) = app.feature(&laptop).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["code"], "sign_in");

    // Bad sign-ups.
    for (email, password) in [("nope", "long enough"), ("a@b.io", "short")] {
        let (status, _) = app
            .call(
                "POST",
                "/api/v1/account",
                &laptop,
                Some(json!({ "email": email, "password": password })),
            )
            .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{email}");
    }

    let email = new_email();
    let (status, body) = app
        .call(
            "POST",
            "/api/v1/account",
            &laptop,
            Some(json!({ "email": email, "password": "correct horse", "device": "mzarch" })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    assert_eq!(body["email"], email.to_lowercase());
    assert_eq!(body["verified"], false);

    // Signed in, address not confirmed yet.
    let (status, body) = app.feature(&laptop).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["code"], "not_verified");

    // The code went to the lowercased address.
    let code = app.code(&email.to_lowercase(), Purpose::Verify);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/verify",
            &laptop,
            Some(json!({ "code": "000000x" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let spaced = format!("{} {}", &code[..3], &code[3..]);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/verify",
            &laptop,
            Some(json!({ "code": spaced })),
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, body) = app.feature(&laptop).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");

    let (status, body) = app.call("GET", "/api/v1/account", &laptop, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["verified"], true);

    // The same address again (in other letter case) is taken.
    let other = app.install().await;
    let (status, body) = app
        .call(
            "POST",
            "/api/v1/account",
            &other,
            Some(json!({ "email": email.to_uppercase(), "password": "another one" })),
        )
        .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["code"], "exists");
}

/// A signed-in, confirmed install and its account's address.
async fn account(app: &App) -> (String, String) {
    let token = app.install().await;
    let email = new_email().to_lowercase();
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account",
            &token,
            Some(json!({ "email": email, "password": "correct horse", "device": "laptop" })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    let code = app.code(&email, Purpose::Verify);
    app.call(
        "POST",
        "/api/v1/account/verify",
        &token,
        Some(json!({ "code": code })),
    )
    .await;
    (token, email)
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn devices_sign_in_and_out() {
    let app = App::new().await;
    let (laptop, email) = account(&app).await;
    let desktop = app.install().await;

    let (status, body) = app
        .call(
            "POST",
            "/api/v1/account/sign-in",
            &desktop,
            Some(json!({ "email": email, "password": "wrong horse", "device": "desktop" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["code"], "wrong_password");
    // An unknown address answers the same.
    let (_, body) = app
        .call(
            "POST",
            "/api/v1/account/sign-in",
            &desktop,
            Some(json!({ "email": "nobody@example.com", "password": "correct horse" })),
        )
        .await;
    assert_eq!(body["code"], "wrong_password");

    let (status, body) = app
        .call(
            "POST",
            "/api/v1/account/sign-in",
            &desktop,
            Some(json!({ "email": email, "password": "correct horse", "device": "desktop\n" })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["verified"], true);
    assert_eq!(app.feature(&desktop).await.0, StatusCode::CREATED);

    let (_, devices) = app
        .call("GET", "/api/v1/account/devices", &laptop, None)
        .await;
    let devices = devices.as_array().unwrap();
    assert_eq!(devices.len(), 2);
    let desktop_row = devices.iter().find(|d| d["name"] == "desktop").unwrap();
    assert_eq!(desktop_row["this"], false);
    assert!(
        devices
            .iter()
            .any(|d| d["name"] == "laptop" && d["this"] == true)
    );

    // The laptop signs the desktop out.
    let uri = format!(
        "/api/v1/account/devices/{}",
        desktop_row["id"].as_str().unwrap()
    );
    let (status, _) = app.call("DELETE", &uri, &laptop, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(app.feature(&desktop).await.1["code"], "sign_in");
    let (status, _) = app.call("DELETE", &uri, &laptop, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Another account cannot sign the laptop out.
    let (stranger, _) = account(&app).await;
    let (_, mine) = app
        .call("GET", "/api/v1/account/devices", &laptop, None)
        .await;
    let laptop_id = mine[0]["id"].as_str().unwrap();
    let (status, _) = app
        .call(
            "DELETE",
            &format!("/api/v1/account/devices/{laptop_id}"),
            &stranger,
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Signing out itself.
    let (status, _) = app
        .call("POST", "/api/v1/account/sign-out", &laptop, None)
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(app.feature(&laptop).await.1["code"], "sign_in");
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn passwords_change_and_reset() {
    let app = App::new().await;
    let (laptop, email) = account(&app).await;
    let desktop = app.install().await;
    app.call(
        "POST",
        "/api/v1/account/sign-in",
        &desktop,
        Some(json!({ "email": email, "password": "correct horse" })),
    )
    .await;

    // Changing the password needs the current one and signs the others out.
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/password",
            &laptop,
            Some(json!({ "current": "wrong horse", "new": "battery staple" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/password",
            &laptop,
            Some(json!({ "current": "correct horse", "new": "battery staple" })),
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(app.feature(&laptop).await.0, StatusCode::CREATED);
    assert_eq!(app.feature(&desktop).await.1["code"], "sign_in");

    // Forgotten password: a code by mail. Unknown addresses answer the same.
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/reset",
            &desktop,
            Some(json!({ "email": "nobody@example.com" })),
        )
        .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/reset",
            &desktop,
            Some(json!({ "email": email })),
        )
        .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let code = app.code(&email, Purpose::Reset);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/reset/confirm",
            &desktop,
            Some(json!({ "email": email, "code": "123", "password": "new password" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/reset/confirm",
            &desktop,
            Some(json!({ "email": email, "code": code, "password": "new password", "device": "desktop" })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    // The desktop is in; the laptop was signed out.
    assert_eq!(app.feature(&desktop).await.0, StatusCode::CREATED);
    assert_eq!(app.feature(&laptop).await.1["code"], "sign_in");
    // The code is used up.
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/reset/confirm",
            &laptop,
            Some(json!({ "email": email, "code": code, "password": "other password" })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/sign-in",
            &laptop,
            Some(json!({ "email": email, "password": "new password" })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn wrong_codes_run_out() {
    let app = App::new().await;
    let token = app.install().await;
    let email = new_email().to_lowercase();
    app.call(
        "POST",
        "/api/v1/account",
        &token,
        Some(json!({ "email": email, "password": "correct horse" })),
    )
    .await;
    let code = app.code(&email, Purpose::Verify);
    let wrong = if code == "000000" { "000001" } else { "000000" };
    for _ in 0..5 {
        let (_, body) = app
            .call(
                "POST",
                "/api/v1/account/verify",
                &token,
                Some(json!({ "code": wrong })),
            )
            .await;
        assert_eq!(body["error"], "wrong code");
    }
    // Even the right code is refused now; a new one works.
    let (_, body) = app
        .call(
            "POST",
            "/api/v1/account/verify",
            &token,
            Some(json!({ "code": code })),
        )
        .await;
    assert_eq!(body["code"], "bad_request");
    let (status, _) = app
        .call("POST", "/api/v1/account/verify/resend", &token, None)
        .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let code = app.code(&email, Purpose::Verify);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/verify",
            &token,
            Some(json!({ "code": code })),
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn deleting_the_account_deletes_its_data() {
    let app = App::new().await;
    let (laptop, email) = account(&app).await;
    let (_, created) = app
        .call(
            "POST",
            "/api/v1/tracks",
            &laptop,
            Some(json!({ "count": 1, "links": ["https://example.com/"] })),
        )
        .await;
    let request = Request::get(format!("/l/{}/0", created["ids"][0].as_str().unwrap()))
        .body(Body::empty())
        .unwrap();
    let response = app.router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FOUND);
    let id = created["ids"][0].as_str().unwrap().to_owned();

    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/delete",
            &laptop,
            Some(json!({ "password": "wrong horse" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account/delete",
            &laptop,
            Some(json!({ "password": "correct horse" })),
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // The device's token and tracking IDs are gone; the address is free.
    assert_eq!(app.feature(&laptop).await.0, StatusCode::UNAUTHORIZED);
    let request = Request::get(format!("/l/{id}/0"))
        .body(Body::empty())
        .unwrap();
    let response = app.router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let fresh = app.install().await;
    let (status, _) = app
        .call(
            "POST",
            "/api/v1/account",
            &fresh,
            Some(json!({ "email": email, "password": "correct horse" })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
}
