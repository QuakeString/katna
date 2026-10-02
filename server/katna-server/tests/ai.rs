// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna AI end to end against a real PostgreSQL and a fake AI service on
//! this computer. Set `KATNA_SERVER_TEST_DATABASE_URL` and run with
//! `--include-ignored`.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use axum::routing::post;
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
}

/// An OpenAI-like service that answers "Rephrased." and keeps what it was
/// asked.
async fn fake_service() -> (String, Arc<Mutex<Vec<Value>>>) {
    let asked: Arc<Mutex<Vec<Value>>> = Arc::default();
    let kept = Arc::clone(&asked);
    let app = Router::new().route(
        "/v1/chat/completions",
        post(move |body: axum::Json<Value>| {
            let kept = Arc::clone(&kept);
            async move {
                kept.lock().unwrap().push(body.0);
                axum::Json(json!({
                    "choices": [{"message": {"content": "Rephrased."}}],
                    "usage": {"prompt_tokens": 1000, "completion_tokens": 500},
                }))
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    (format!("http://{address}/v1"), asked)
}

fn service(base: &str) -> AiService {
    AiService {
        provider: "other",
        base: base.to_owned(),
        model: "fake-model".into(),
        key: Secret("test-key".into()),
    }
}

impl App {
    async fn new(ai: AiConfig) -> Self {
        let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
        let db = Db::connect(&url).unwrap();
        db.migrate().await.unwrap();
        let config = Config {
            database_url: url,
            installs_per_hour: 1000,
            ai,
            ..Config::default()
        };
        let outbox = Arc::default();
        let router = router(AppState::with_mailer(
            db.clone(),
            config,
            Mailer::Memory(Arc::clone(&outbox)),
        ));
        Self { router, db, outbox }
    }

    async fn call(&self, uri: &str, token: &str, body: Value) -> (StatusCode, Value) {
        let mut request = Request::post(uri).header(header::CONTENT_TYPE, "application/json");
        if !token.is_empty() {
            request = request.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }
        let request = request.body(Body::from(body.to_string())).unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    /// A signed-in, confirmed install's token and its account's ID.
    async fn account(&self) -> (String, String) {
        let (status, body) = self.call("/api/v1/installs", "", json!({})).await;
        assert_eq!(status, StatusCode::CREATED);
        let token = body["token"].as_str().unwrap().to_owned();
        let email = format!("someone.{}@example.com", katna_server::ids::new_id());
        let (status, _) = self
            .call(
                "/api/v1/account",
                &token,
                json!({ "email": email, "password": "correct horse", "device": "laptop" }),
            )
            .await;
        assert_eq!(status, StatusCode::CREATED);
        let code = self
            .outbox
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|sent| sent.to == email && sent.purpose == Purpose::Verify)
            .map(|sent| sent.code.clone())
            .unwrap();
        let (status, _) = self
            .call("/api/v1/account/verify", &token, json!({ "code": code }))
            .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        let account = self.db.account_by_email(&email).await.unwrap().unwrap().id;
        (token, account)
    }

    async fn rephrase(&self, token: &str) -> (StatusCode, Value) {
        self.call(
            "/api/v1/ai/rephrase",
            token,
            json!({ "text": "pls send the file asap", "tone": "formal" }),
        )
        .await
    }
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn rephrases_in_the_free_month_through_the_fallback() {
    let (base, asked) = fake_service().await;
    let app = App::new(AiConfig {
        // Nothing listens on port 9: the fallback answers.
        services: vec![service("http://127.0.0.1:9/v1"), service(&base)],
        ..AiConfig::default()
    })
    .await;
    let (token, _) = app.account().await;

    let (status, body) = app.rephrase(&token).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["text"], "Rephrased.");
    assert_eq!(body["plan"], json!({ "kind": "trial", "days_left": 30 }));

    // The service got the prompt built here, with the text in it.
    let asked = asked.lock().unwrap().clone();
    assert_eq!(asked.len(), 1);
    assert_eq!(asked[0]["model"], "fake-model");
    let user = asked[0]["messages"][1]["content"].as_str().unwrap();
    assert!(user.contains("pls send the file asap"), "{user}");

    // Too little to finish: no suggestion, nothing asked.
    let (status, body) = app
        .call("/api/v1/ai/complete", &token, json!({ "before": "Hi" }))
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["text"], "");

    // Bad requests.
    let (status, _) = app
        .call(
            "/api/v1/ai/rephrase",
            &token,
            json!({ "text": "x", "tone": "loud" }),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn needs_an_account_and_payment_after_the_free_month() {
    let (base, _) = fake_service().await;
    let app = App::new(AiConfig {
        services: vec![service(&base)],
        trial_days: 0,
        ..AiConfig::default()
    })
    .await;

    // Not signed in.
    let (status, body) = app.call("/api/v1/installs", "", json!({})).await;
    assert_eq!(status, StatusCode::CREATED);
    let token = body["token"].as_str().unwrap();
    let (status, body) = app.rephrase(token).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["code"], "sign_in");

    // The free month is over.
    let (token, account) = app.account().await;
    let (status, body) = app.rephrase(&token).await;
    assert_eq!(status, StatusCode::PAYMENT_REQUIRED);
    assert_eq!(body["code"], "pay");

    // Paid.
    let now = katna_server::db::now_ms();
    app.db
        .set_ai_paid_until(&account, now + 86_400_000, now)
        .await
        .unwrap();
    let (status, body) = app.rephrase(&token).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["plan"], json!({ "kind": "paid" }));
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn stops_at_the_monthly_cap() {
    let (base, asked) = fake_service().await;
    let app = App::new(AiConfig {
        services: vec![service(&base)],
        // One answer costs 300 millionths of a dollar.
        account_cap_micros: 500,
        ..AiConfig::default()
    })
    .await;
    let (token, _) = app.account().await;
    assert_eq!(app.rephrase(&token).await.0, StatusCode::OK);
    assert_eq!(app.rephrase(&token).await.0, StatusCode::OK);
    let (status, body) = app.rephrase(&token).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["code"], "too_many");
    assert_eq!(asked.lock().unwrap().len(), 2);

    // Another account still may.
    let (other, _) = app.account().await;
    assert_eq!(app.rephrase(&other).await.0, StatusCode::OK);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn off_without_a_service() {
    let app = App::new(AiConfig::default()).await;
    let (token, _) = app.account().await;
    let (status, body) = app.rephrase(&token).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["code"], "busy");
}
