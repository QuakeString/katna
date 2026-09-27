// SPDX-License-Identifier: GPL-3.0-or-later

//! The server end to end against a real PostgreSQL. Set
//! `KATNA_SERVER_TEST_DATABASE_URL` (CI's `server` job does) and run with
//! `--include-ignored`.

use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use katna_server::db::Db;
use katna_server::{AppState, Config, router};
use serde_json::{Value, json};
use tower::ServiceExt;

const NEEDS_DB: &str = "needs PostgreSQL in KATNA_SERVER_TEST_DATABASE_URL";
const FIREFOX: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0";

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
    router(AppState::new(db, config))
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

async fn register(app: &Router) -> String {
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
