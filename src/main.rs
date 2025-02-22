use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::Html,
    routing::get,
    Json,
    Router,
};
use http::StatusCode;
use tower_http::limit::RequestBodyLimitLayer;
use serde::Serialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use whisper_trtllm_rs::Whisper;

#[derive(Serialize)]
struct Detection {
    language: String,
}

#[tokio::main]
async fn main() {
    /*
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    */

    // build our application with some routes

    let whisper = Arc::new(Whisper::load("models/whisper_turbo_int8").unwrap());

    let app = Router::new()
        .route("/v1/audio/detections", get(show_form).post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            1024 * 1024 * 1024, /* 1GB */
        ));
        //.layer(tower_http::trace::TraceLayer::new_for_http()


    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    //tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/v1/audio/detections" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(mut multipart: Multipart) -> Result<Json<Detection>, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        if let Some(name) = field.name() {
            if name == "file" {
                let detection = Detection { language: "en".to_string() };
                return Ok(Json(detection));
            }
        }
    }
    Err((StatusCode::BAD_REQUEST, "No file found".to_string()))
}