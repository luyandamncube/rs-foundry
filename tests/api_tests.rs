// tests\api_tests.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use rs_foundry::api::{routes, state::AppState};
use rs_foundry::config::settings::Settings;
use tower::util::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let state = AppState {
        settings: Settings::default(),
    };

    let app = routes::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn health_endpoint_returns_expected_json_body() {
    let state = AppState {
        settings: Settings::default(),
    };

    let app = routes::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK);

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "rs-foundry");
}

#[tokio::test]
async fn unknown_route_returns_not_found() {
    let state = AppState {
        settings: Settings::default(),
    };

    let app = routes::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
