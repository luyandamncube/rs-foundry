// tests\api_tests.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use rs_foundry::api::{routes, state::AppState};
use rs_foundry::config::settings::Settings;
use rs_foundry::io::postgres::create_pool;
use tower::util::ServiceExt;

async fn test_app() -> axum::Router {
    let settings = Settings::default();
    let db_pool = create_pool(&settings.database_url()).await.unwrap();

    let state = AppState {
        settings,
        db_pool,
    };

    routes::router(state)
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let app = test_app().await;

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
async fn ready_endpoint_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/ready")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn bronze_ref_job_endpoint_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jobs/bronze/ref")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"requested_by":"test"}"#))
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
    assert_eq!(json["job_name"], "bronze_ref");
    assert_eq!(json["source_name"], "ref_example");
}

#[tokio::test]
async fn bronze_daily_job_endpoint_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jobs/bronze/daily")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"requested_by":"test"}"#))
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
    assert_eq!(json["job_name"], "bronze_daily");
    assert_eq!(json["source_name"], "daily_example");
}

#[tokio::test]
async fn unknown_route_returns_not_found() {
    let app = test_app().await;

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


#[tokio::test]
async fn runs_endpoint_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/runs")
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
    assert!(json.get("runs").is_some());
}
