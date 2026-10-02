// SPDX-License-Identifier: GPL-3.0-or-later

//! The admin page end to end against a real PostgreSQL. Set
//! `KATNA_SERVER_TEST_DATABASE_URL` and run with `--include-ignored`.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use katna_server::config::{AiConfig, AiService, Secret};
use katna_server::db::Db;
use katna_server::mailer::{Mailer, Purpose, SentCode};
use katna_server::{AppState, Config, router};
use serde_json::{Value, json};
use tower::ServiceExt;

const NEEDS_DB: &str = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL";

struct App {
    router: Router,
    db: Db,
    outbox: Arc<Mutex<Vec<SentCode>>>,
    admin: String,
}

fn key(provider: &'static str, base: &str, model: &str) -> AiService {
    AiService {
        provider,
        base: base.into(),
        model: model.into(),
        key: Secret(format!("{provider}-secret")),
    }
}

impl App {
    async fn new(with_admin: bool) -> Self {
        let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
        let db = Db::connect(&url).unwrap();
        db.migrate().await.unwrap();
        let admin = format!("admin.{}@example.com", katna_server::ids::new_id());
        let gemini = key(
            "gemini",
            "https://generativelanguage.googleapis.com",
            "gemini-2.5-flash-lite",
        );
        let config = Config {
            database_url: url,
            installs_per_hour: 1000,
            admin_emails: if with_admin {
                vec![admin.clone()]
            } else {
                vec![]
            },
            ai: AiConfig {
                services: vec![gemini.clone()],
                keys: vec![
                    gemini,
                    key(
                        "mistral",
                        "https://api.mistral.ai/v1",
                        "mistral-small-latest",
                    ),
                ],
                ..AiConfig::default()
            },
            ..Config::default()
        };
        let outbox = Arc::default();
        let router = router(AppState::with_mailer(
            db.clone(),
            config,
            Mailer::Memory(Arc::clone(&outbox)),
        ));
        Self {
            router,
            db,
            outbox,
            admin,
        }
    }

    /// A call, with the page's header unless `from_page` is false, and the
    /// session `cookie`; the status, body and any cookie set.
    async fn call(
        &self,
        method: &str,
        uri: &str,
        bearer: Option<&str>,
        cookie: Option<&str>,
        from_page: bool,
        body: Option<Value>,
    ) -> (StatusCode, Value, Option<String>) {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(token) = bearer {
            request = request.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        if from_page {
            request = request.header("x-katna-admin", "1");
        }
        let request = match body {
            Some(body) => request
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => request.body(Body::empty()),
        }
        .unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let set = response
            .headers()
            .get(header::SET_COOKIE)
            .map(|v| v.to_str().unwrap().to_owned());
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
            set,
        )
    }

    fn last_code(&self, to: &str, purpose: Purpose) -> Option<String> {
        self.outbox
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|sent| sent.to == to && sent.purpose == purpose)
            .map(|sent| sent.code.clone())
    }

    /// A confirmed Katna account for `email`.
    async fn account(&self, email: &str) {
        let (status, body, _) = self
            .call("POST", "/api/v1/installs", None, None, false, None)
            .await;
        assert_eq!(status, StatusCode::CREATED);
        let token = body["token"].as_str().unwrap().to_owned();
        let (status, _, _) = self
            .call(
                "POST",
                "/api/v1/account",
                Some(&token),
                None,
                false,
                Some(json!({ "email": email, "password": "correct horse", "device": "x" })),
            )
            .await;
        assert_eq!(status, StatusCode::CREATED);
        let code = self.last_code(email, Purpose::Verify).unwrap();
        let (status, _, _) = self
            .call(
                "POST",
                "/api/v1/account/verify",
                Some(&token),
                None,
                false,
                Some(json!({ "code": code })),
            )
            .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
    }

    /// Sets the admin page password on the server, as
    /// `katna-server admin-password` does.
    async fn admin_password(&self, password: &str) {
        let hash = katna_server::auth::hash_password(password.into())
            .await
            .unwrap();
        self.db
            .set_admin_password(&self.admin, &hash, katna_server::db::now_ms())
            .await
            .unwrap();
    }

    /// Signs the admin in; the cookie to send.
    async fn sign_in(&self) -> String {
        let (status, _, _) = self
            .call(
                "POST",
                "/admin/api/sign-in",
                None,
                None,
                true,
                Some(json!({ "email": self.admin, "password": "admin battery" })),
            )
            .await;
        assert_eq!(status, StatusCode::ACCEPTED);
        let code = self.last_code(&self.admin, Purpose::Admin).unwrap();
        let (status, _, set) = self
            .call(
                "POST",
                "/admin/api/code",
                None,
                None,
                true,
                Some(json!({ "email": self.admin, "code": code })),
            )
            .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let set = set.unwrap();
        assert!(
            set.contains("HttpOnly") && set.contains("Secure") && set.contains("SameSite=Strict")
        );
        set.split(';').next().unwrap().to_owned()
    }
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn admins_sign_in_with_a_mailed_code_and_change_katna_ai() {
    let app = App::new(true).await;
    // A Katna account with the admin's address is not the admin.
    app.account(&app.admin).await;
    for password in ["correct horse", "admin battery"] {
        let (status, _, _) = app
            .call(
                "POST",
                "/admin/api/sign-in",
                None,
                None,
                true,
                Some(json!({ "email": app.admin, "password": password })),
            )
            .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    app.admin_password("admin battery").await;

    // The page loads; its data needs a session.
    let (status, _, _) = app.call("GET", "/admin", None, None, false, None).await;
    assert_eq!(status, StatusCode::OK);
    let (status, body, _) = app
        .call("GET", "/admin/api/state", None, None, false, None)
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["code"], "admin_sign_in");

    // Wrong password: no code.
    let (status, _, _) = app
        .call(
            "POST",
            "/admin/api/sign-in",
            None,
            None,
            true,
            Some(json!({ "email": app.admin, "password": "wrong one" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(app.last_code(&app.admin, Purpose::Admin).is_none());

    let cookie = app.sign_in().await;
    // A code works once.
    let used = app.last_code(&app.admin, Purpose::Admin).unwrap();
    let (status, body, _) = app
        .call(
            "POST",
            "/admin/api/code",
            None,
            None,
            true,
            Some(json!({ "email": app.admin, "code": used })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["code"], "code_expired");
    let (status, body, _) = app
        .call("GET", "/admin/api/state", None, Some(&cookie), false, None)
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["email"], app.admin);
    assert_eq!(body["settings"]["main"]["provider"], "gemini");
    let keys: Vec<_> = body["services"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["has_key"] == true)
        .map(|s| s["id"].as_str().unwrap())
        .collect();
    assert_eq!(keys, ["gemini", "mistral"]);
    // Keys never leave the server.
    assert!(!body.to_string().contains("secret"));

    // A change without the page's header is refused.
    let mut settings = body["settings"].clone();
    settings["fallback"] = json!({ "provider": "mistral", "model": "mistral-small-latest" });
    settings["budget_micros"] = json!(20_000_000);
    let (status, _, _) = app
        .call(
            "POST",
            "/admin/api/settings",
            None,
            Some(&cookie),
            false,
            Some(settings.clone()),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, body, _) = app
        .call(
            "POST",
            "/admin/api/settings",
            None,
            Some(&cookie),
            true,
            Some(settings.clone()),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["settings"]["fallback"]["provider"], "mistral");
    assert_eq!(body["settings"]["budget_micros"], 20_000_000);

    // A service without a key can't be chosen.
    settings["main"] = json!({ "provider": "openai", "model": "gpt-5-mini" });
    let (status, body, _) = app
        .call(
            "POST",
            "/admin/api/settings",
            None,
            Some(&cookie),
            true,
            Some(settings),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["code"], "bad_settings");

    // Signed out, the cookie no longer works.
    let (status, _, _) = app
        .call(
            "POST",
            "/admin/api/sign-out",
            None,
            Some(&cookie),
            true,
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _, _) = app
        .call("GET", "/admin/api/state", None, Some(&cookie), false, None)
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn other_accounts_get_no_code() {
    let app = App::new(true).await;
    let someone = format!("someone.{}@example.com", katna_server::ids::new_id());
    app.account(&someone).await;
    let (status, _, _) = app
        .call(
            "POST",
            "/admin/api/sign-in",
            None,
            None,
            true,
            Some(json!({ "email": someone, "password": "correct horse" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(app.last_code(&someone, Purpose::Admin).is_none());
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn no_admins_no_page() {
    let app = App::new(false).await;
    for path in ["/admin", "/admin/app.js", "/admin/api/state"] {
        let (status, _, _) = app.call("GET", path, None, None, true, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{path}");
    }
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn the_first_password_is_chosen_on_the_page_after_a_mailed_code() {
    let app = App::new(true).await;
    let status = |body: Value| body["setup"].as_bool();
    let (_, body, _) = app
        .call("GET", "/admin/api/status", None, None, false, None)
        .await;
    assert_eq!(status(body), Some(true));

    // Anyone may ask; only the admin's address gets a code.
    for email in ["stranger@example.com", app.admin.as_str()] {
        let (code, _, _) = app
            .call(
                "POST",
                "/admin/api/setup",
                None,
                None,
                true,
                Some(json!({ "email": email })),
            )
            .await;
        assert_eq!(code, StatusCode::ACCEPTED);
    }
    assert!(
        app.last_code("stranger@example.com", Purpose::AdminSetup)
            .is_none()
    );
    let code = app.last_code(&app.admin, Purpose::AdminSetup).unwrap();

    // A wrong code sets nothing.
    let finish =
        |code: String| json!({ "email": app.admin, "code": code, "password": "admin battery" });
    let (status_code, body, _) = app
        .call(
            "POST",
            "/admin/api/setup/finish",
            None,
            None,
            true,
            Some(finish("000000".into())),
        )
        .await;
    assert_eq!(status_code, StatusCode::BAD_REQUEST, "{body}");

    let (status_code, _, set) = app
        .call(
            "POST",
            "/admin/api/setup/finish",
            None,
            None,
            true,
            Some(finish(code)),
        )
        .await;
    assert_eq!(status_code, StatusCode::NO_CONTENT);
    let cookie = set.unwrap().split(';').next().unwrap().to_owned();
    let (status_code, _, _) = app
        .call("GET", "/admin/api/state", None, Some(&cookie), false, None)
        .await;
    assert_eq!(status_code, StatusCode::OK);
    let (_, body, _) = app
        .call("GET", "/admin/api/status", None, None, false, None)
        .await;
    assert_eq!(status(body), Some(false));

    // Once set, the page can't choose it again; signing in works.
    let (code, _, _) = app
        .call(
            "POST",
            "/admin/api/setup",
            None,
            None,
            true,
            Some(json!({ "email": app.admin })),
        )
        .await;
    assert_eq!(code, StatusCode::ACCEPTED);
    assert_eq!(
        app.outbox
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.purpose == Purpose::AdminSetup)
            .count(),
        1
    );
    app.sign_in().await;
}
