// SPDX-License-Identifier: GPL-3.0-or-later

//! The server end to end against a real PostgreSQL. Set
//! `KATNA_SERVER_TEST_DATABASE_URL` (CI's `server` job does) and run with
//! `--include-ignored`.

use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

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
const FIREFOX: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0";

/// Codes "mailed" by every test's server.
static OUTBOX: LazyLock<Arc<Mutex<Vec<SentCode>>>> = LazyLock::new(Arc::default);

/// The last code mailed to `to` for `purpose`.
fn code_for(to: &str, purpose: Purpose) -> String {
    OUTBOX
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|sent| sent.to == to && sent.purpose == purpose)
        .map(|sent| sent.code.clone())
        .expect("a code was mailed")
}

async fn app() -> Router {
    let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
    let db = Db::connect(&url).unwrap();
    db.migrate().await.unwrap();
    let config = Config {
        database_url: url,
        installs_per_hour: 1000,
        daily_limit: 50,
        ..Config::default()
    };
    router(AppState::with_mailer(
        db,
        config,
        Mailer::Memory(OUTBOX.clone()),
    ))
}

async fn send(
    app: &Router,
    request: Request<Body>,
) -> (StatusCode, axum::http::HeaderMap, Vec<u8>) {
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, headers, body)
}

/// A new install token, signed in to a new account with a confirmed
/// address.
async fn register(app: &Router) -> String {
    register_as(app).await.0
}

/// A new install token, signed in to a new account with a confirmed
/// address, and that address.
async fn register_as(app: &Router) -> (String, String) {
    let token = new_install(app).await;
    let email = format!("{}@example.com", katna_server::ids::new_id());
    let (status, _, body) = send(
        app,
        authed(
            "POST",
            "/api/v1/account",
            &token,
            Some(json!({ "email": email, "password": "correct horse", "device": "test" })),
        ),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&body)
    );
    let code = code_for(&email, Purpose::Verify);
    let (status, _, _) = send(
        app,
        authed(
            "POST",
            "/api/v1/account/verify",
            &token,
            Some(json!({ "code": code })),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    (token, email)
}

/// A new install token, not signed in.
async fn new_install(app: &Router) -> String {
    let (status, _, body) = send(
        app,
        Request::post("/api/v1/installs")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let body: Value = serde_json::from_slice(&body).unwrap();
    body["token"].as_str().unwrap().to_owned()
}

fn authed(method: &str, uri: &str, token: &str, body: Option<Value>) -> Request<Body> {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {token}"));
    match body {
        Some(body) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    }
}

async fn create(app: &Router, token: &str, count: u32, links: &[&str]) -> Vec<String> {
    let (status, _, body) = send(
        app,
        authed(
            "POST",
            "/api/v1/tracks",
            token,
            Some(json!({ "count": count, "links": links })),
        ),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&body)
    );
    let body: Value = serde_json::from_slice(&body).unwrap();
    body["ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_owned())
        .collect()
}

fn get(uri: &str, user_agent: &str) -> Request<Body> {
    Request::get(uri)
        .header(header::USER_AGENT, user_agent)
        .body(Body::empty())
        .unwrap()
}

/// Reads server-sent events until `count` have arrived.
async fn read_events(app: &Router, token: &str, after: i64, count: usize) -> Vec<Value> {
    let request = Request::get("/api/v1/events")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header("last-event-id", after.to_string())
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/event-stream"
    );
    let mut body = response.into_body();
    let mut text = String::new();
    let mut events = Vec::new();
    while events.len() < count {
        let frame = tokio::time::timeout(Duration::from_secs(5), body.frame())
            .await
            .expect("an event within 5 s")
            .unwrap()
            .unwrap();
        if let Ok(data) = frame.into_data() {
            text.push_str(&String::from_utf8_lossy(&data));
        }
        while let Some(end) = text.find("\n\n") {
            let block: String = text.drain(..end + 2).collect();
            if let Some(data) = block.lines().find_map(|line| line.strip_prefix("data: ")) {
                events.push(serde_json::from_str(data).unwrap());
            }
        }
    }
    events
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn pixel_and_link_events_reach_the_stream() {
    let app = app().await;
    let token = register(&app).await;
    let ids = create(&app, &token, 2, &["https://example.com/proposal?v=2"]).await;
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);

    // Within the scanner window: labelled as a scanner.
    let (status, headers, body) = send(&app, get(&format!("/o/{}.png", ids[0]), FIREFOX)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers[header::CONTENT_TYPE], "image/png");
    assert!(
        headers[header::CACHE_CONTROL]
            .to_str()
            .unwrap()
            .contains("no-store")
    );
    assert_eq!(body, katna_server::routes::PIXEL);

    let (status, headers, _) = send(&app, get(&format!("/l/{}/0", ids[1]), FIREFOX)).await;
    assert_eq!(status, StatusCode::FOUND);
    assert_eq!(
        headers[header::LOCATION],
        "https://example.com/proposal?v=2"
    );

    let (status, _, _) = send(&app, get(&format!("/o/{}.png", ids[1]), "Mozilla/5.0")).await;
    assert_eq!(status, StatusCode::OK);

    let events = read_events(&app, &token, 0, 3).await;
    assert_eq!(events[0]["id"], ids[0].as_str());
    assert_eq!(events[0]["kind"], "open");
    assert_eq!(events[0]["source"], "scanner");
    assert_eq!(events[1]["kind"], "click");
    assert_eq!(events[1]["link"], 0);
    assert_eq!(events[2]["source"], "apple_proxy");
    assert!(events[0]["at"].as_i64().unwrap() > 0);

    // Resuming after the second event gives only the third.
    let second = events[1]["seq"].as_i64().unwrap();
    let resumed = read_events(&app, &token, second, 1).await;
    assert_eq!(resumed[0]["seq"], events[2]["seq"]);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn live_events_arrive_while_connected() {
    let app = app().await;
    let token = register(&app).await;
    let ids = create(&app, &token, 1, &[]).await;
    let reader = {
        let app = app.clone();
        let token = token.clone();
        tokio::spawn(async move { read_events(&app, &token, 0, 1).await })
    };
    tokio::time::sleep(Duration::from_millis(300)).await;
    send(&app, get(&format!("/o/{}.png", ids[0]), FIREFOX)).await;
    let events = reader.await.unwrap();
    assert_eq!(events[0]["id"], ids[0].as_str());
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn installs_see_only_their_own_events() {
    let app = app().await;
    let mine = register(&app).await;
    let theirs = register(&app).await;
    let their_ids = create(&app, &theirs, 1, &[]).await;
    let my_ids = create(&app, &mine, 1, &[]).await;
    send(&app, get(&format!("/o/{}.png", their_ids[0]), FIREFOX)).await;
    send(&app, get(&format!("/o/{}.png", my_ids[0]), FIREFOX)).await;
    let events = read_events(&app, &mine, 0, 1).await;
    assert_eq!(events[0]["id"], my_ids[0].as_str());

    // Someone else's ID cannot be deleted.
    let (status, _, _) = send(
        &app,
        authed(
            "DELETE",
            &format!("/api/v1/tracks/{}", their_ids[0]),
            &mine,
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn never_an_open_redirect() {
    let app = app().await;
    let token = register(&app).await;
    let ids = create(&app, &token, 1, &["https://example.com/"]).await;
    for uri in [
        format!("/l/{}/1", ids[0]),
        format!("/l/{}/0", "0".repeat(32)),
        "/l/not-an-id/0".to_owned(),
        "/l/https%3A%2F%2Fevil.example/0".to_owned(),
    ] {
        let (status, headers, _) = send(&app, get(&uri, FIREFOX)).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
        assert!(headers.get(header::LOCATION).is_none());
    }
    for bad in [
        "javascript:alert(1)",
        "data:text/html,x",
        "//evil.example",
        "https://a b",
    ] {
        let (status, _, _) = send(
            &app,
            authed(
                "POST",
                "/api/v1/tracks",
                &token,
                Some(json!({ "count": 1, "links": [bad] })),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{bad}");
    }
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn tokens_limits_and_forgetting() {
    let app = app().await;
    // No or wrong token.
    let (status, _, _) = send(&app, authed("GET", "/api/v1/events", &"0".repeat(64), None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _, _) = send(
        &app,
        Request::post("/api/v1/tracks")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"count":1}"#))
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let token = register(&app).await;
    let (status, _, _) = send(
        &app,
        authed(
            "POST",
            "/api/v1/tracks",
            &token,
            Some(json!({ "count": 0 })),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    // The test daily limit is 50.
    create(&app, &token, 40, &[]).await;
    let (status, _, _) = send(
        &app,
        authed(
            "POST",
            "/api/v1/tracks",
            &token,
            Some(json!({ "count": 11 })),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);

    // Unknown IDs still get the picture.
    let (status, _, _) = send(&app, get(&format!("/o/{}.png", "f".repeat(32)), FIREFOX)).await;
    assert_eq!(status, StatusCode::OK);

    // Forgetting the install removes its IDs.
    let ids = create(&app, &token, 1, &["https://example.com/"]).await;
    let (status, _, _) = send(&app, authed("DELETE", "/api/v1/installs/me", &token, None)).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _, _) = send(&app, get(&format!("/l/{}/0", ids[0]), FIREFOX)).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _, _) = send(&app, authed("DELETE", "/api/v1/installs/me", &token, None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn about_page_and_health() {
    let app = app().await;
    let (status, _, body) = send(&app, get("/", FIREFOX)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&body).contains("never receives the subject"));
    let (status, _, body) = send(&app, get("/healthz", FIREFOX)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, b"ok");
}

/// A stand-in for LibreTranslate: "translates" by putting the target's
/// code in front, and remembers what it was sent.
async fn fake_libretranslate() -> (String, std::sync::Arc<std::sync::Mutex<Vec<Value>>>) {
    use axum::routing::{get, post};
    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let kept = seen.clone();
    let app = Router::new()
        .route(
            "/languages",
            get(|| async {
                axum::Json(json!([{"code": "es", "name": "Spanish", "targets": ["en"]}]))
            }),
        )
        .route(
            "/translate",
            post(move |axum::Json(body): axum::Json<Value>| {
                let kept = kept.clone();
                async move {
                    kept.lock().unwrap().push(body.clone());
                    let text = format!(
                        "[{}] {}",
                        body["target"].as_str().unwrap(),
                        body["q"].as_str().unwrap()
                    );
                    axum::Json(json!({ "translatedText": text }))
                }
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    (format!("http://{address}"), seen)
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn translation_passes_through_for_signed_in_accounts_only() {
    let (translate_url, seen) = fake_libretranslate().await;
    let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
    let db = Db::connect(&url).unwrap();
    db.migrate().await.unwrap();
    let app = router(AppState::with_mailer(
        db,
        Config {
            database_url: url,
            installs_per_hour: 1000,
            translate_url,
            translations_per_day: 3,
            ..Config::default()
        },
        Mailer::Memory(OUTBOX.clone()),
    ));
    let request =
        json!({"q": "Hola", "source": "es", "target": "en", "format": "html", "api_key": "x"});

    // No token, no translation.
    let (status, _, _) = send(
        &app,
        Request::post("/api/v1/translate")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(request.to_string()))
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // A computer not signed in to an account is asked to sign in.
    let signed_out = new_install(&app).await;
    let (status, _, body) = send(
        &app,
        authed(
            "POST",
            "/api/v1/translate",
            &signed_out,
            Some(request.clone()),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap()["code"],
        "sign_in"
    );
    assert!(seen.lock().unwrap().is_empty());

    let token = register(&app).await;
    let (status, _, body) = send(&app, authed("GET", "/api/v1/languages", &token, None)).await;
    assert_eq!(status, StatusCode::OK);
    let languages: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(languages[0]["targets"][0], "en");

    let (status, _, body) = send(
        &app,
        authed("POST", "/api/v1/translate", &token, Some(request.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
    let answer: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(answer["translatedText"], "[en] Hola");
    // Plain text only, and nothing but the text and the languages.
    assert_eq!(
        seen.lock().unwrap()[0],
        json!({"q": "Hola", "source": "es", "target": "en", "format": "text"})
    );

    let bad = json!({"q": "Hola", "source": "es; x", "target": "en"});
    let (status, _, _) = send(&app, authed("POST", "/api/v1/translate", &token, Some(bad))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // Three a day in this test.
    let (status, _, _) = send(
        &app,
        authed("POST", "/api/v1/translate", &token, Some(request.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _, _) = send(
        &app,
        authed("POST", "/api/v1/translate", &token, Some(request)),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
#[ignore = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL"]
async fn limits_count_per_account_and_links_are_stored_once() {
    let app = app().await;
    let (laptop, email) = register_as(&app).await;
    let desktop = new_install(&app).await;
    let (status, _, _) = send(
        &app,
        authed(
            "POST",
            "/api/v1/account/sign-in",
            &desktop,
            Some(json!({ "email": email, "password": "correct horse", "device": "desktop" })),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // The test daily limit is 50, for the account's devices together.
    let links = ["https://example.com/a", "https://example.com/b"];
    let ids = create(&app, &laptop, 40, &links).await;
    let tracks = |token: &str, count: u32, links: Value| {
        authed(
            "POST",
            "/api/v1/tracks",
            token,
            Some(json!({ "count": count, "links": links })),
        )
    };
    let (status, _, _) = send(&app, tracks(&desktop, 11, json!([]))).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    // Parallel requests cannot pass the limit together.
    let answers = tokio::join!(
        send(&app, tracks(&desktop, 5, json!([]))),
        send(&app, tracks(&desktop, 5, json!([]))),
        send(&app, tracks(&laptop, 5, json!([]))),
        send(&app, tracks(&laptop, 5, json!([]))),
    );
    let created = [answers.0, answers.1, answers.2, answers.3]
        .iter()
        .filter(|(status, _, _)| *status == StatusCode::CREATED)
        .count();
    assert_eq!(created, 2, "10 left for today");

    // Too many bytes of links in one request.
    let long = format!("https://example.com/{}", "x".repeat(4000));
    let (status, _, _) = send(&app, tracks(&laptop, 1, json!(vec![long; 70]))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // Every copy follows the shared links, each to its own.
    for id in [&ids[0], &ids[39]] {
        for (n, target) in links.iter().enumerate() {
            let (status, headers, _) = send(&app, get(&format!("/l/{id}/{n}"), FIREFOX)).await;
            assert_eq!(status, StatusCode::FOUND);
            assert_eq!(headers[header::LOCATION], *target);
        }
        let (status, _, _) = send(&app, get(&format!("/l/{id}/2"), FIREFOX)).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    // IDs stored before links were shared still redirect.
    let url = std::env::var("KATNA_SERVER_TEST_DATABASE_URL").expect(NEEDS_DB);
    let (client, connection) = tokio_postgres::connect(&url, tokio_postgres::NoTls)
        .await
        .unwrap();
    tokio::spawn(connection);
    client
        .execute(
            "UPDATE tracks t SET links = s.links, link_set = NULL
             FROM link_sets s WHERE s.id = t.link_set AND t.id = $1",
            &[&ids[1]],
        )
        .await
        .unwrap();
    let (status, headers, _) = send(&app, get(&format!("/l/{}/1", ids[1]), FIREFOX)).await;
    assert_eq!(status, StatusCode::FOUND);
    assert_eq!(headers[header::LOCATION], links[1]);
}
