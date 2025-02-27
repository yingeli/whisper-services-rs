use axum::{
    extract::{State, DefaultBodyLimit, Multipart},
    response::Html,
    routing::get,
    Json,
    Router,
};
use http::StatusCode;
use tower_http::limit::RequestBodyLimitLayer;
use serde::Serialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use whisper_trtllm_rs::{Whisper, Transcript};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;
use futures::stream::TryStreamExt;
use axum::debug_handler;

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

    let whisper = Arc::new(Whisper::load("models/turbo").unwrap());

    let app = Router::new()
        .route("/v1/audio/detections", get(show_detection_form).post(detect))
        .route("/v1/audio/transcriptions", get(show_transcription_form).post(transcribe))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            1024 * 1024 * 1024, /* 1GB */
        ))
        .with_state(whisper);
        //.layer(tower_http::trace::TraceLayer::new_for_http()


    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    //tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_detection_form() -> Html<&'static str> {
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

async fn show_transcription_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/v1/audio/transcriptions" method="post" enctype="multipart/form-data">
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

async fn detect(State(whisper): State<Arc<Whisper>>, mut multipart: Multipart) -> Result<Json<Detection>, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let Some(name) = field.name() else {
            continue;
        };

        if name == "file" {
            // Convert field into AsyncRead stream
            let stream = field.into_stream().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
            let mut reader = StreamReader::new(stream);

            // Verify WAV header
            let mut header = [0; 44];
            reader.read_exact(&mut header).await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            // Verify WAV format
            if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
                return Err((StatusCode::BAD_REQUEST, "Invalid WAV format".to_string()));
            }   

            // Parse WAV header fields
            let channels = u16::from_le_bytes([header[22], header[23]]);
            let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
            let bits_per_sample = u16::from_le_bytes([header[34], header[35]]);

            if channels != 1 || sample_rate != 16000 || bits_per_sample != 16 {
                return Err((StatusCode::BAD_REQUEST, "Only mono 16-bit 16kHz WAV files are supported".to_string()));
            }

            let language = whisper
                .detect_language(reader).await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let detection = Detection { language };                

            return Ok(Json(detection));
        }
    }
    Err((StatusCode::BAD_REQUEST, "No file found".to_string()))
}

#[debug_handler]
async fn transcribe(State(whisper): State<Arc<Whisper>>, mut multipart: Multipart) -> Result<Json<Transcript>, (StatusCode, String)> {
    let mut language = None;
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let Some(name) = field.name() else {
            continue;
        };
        match name {
            "language" => {
                let field_text = field.text().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                language = Some(field_text);
            },
            "file" => {
                // Convert field into AsyncRead stream
                let stream = field.into_stream().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));         
                let mut reader = StreamReader::new(stream);
                //let time = std::time::Instant::now();
                //let bytes = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                //let mut reader = std::io::Cursor::new(bytes);
                //println!("Read to Bytes: {}ms", time.elapsed().as_millis());

                // Verify WAV header
                let mut header = [0; 44];
                reader.read_exact(&mut header).await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

                // Verify WAV format
                if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
                    return Err((StatusCode::BAD_REQUEST, "Invalid WAV format".to_string()));
                }   

                // Parse WAV header fields
                let channels = u16::from_le_bytes([header[22], header[23]]);
                let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
                let bits_per_sample = u16::from_le_bytes([header[34], header[35]]);

                if channels != 1 || sample_rate != 16000 || bits_per_sample != 16 {
                    return Err((StatusCode::BAD_REQUEST, "Only mono 16-bit 16kHz WAV files are supported".to_string()));
                }

                let prompt = "Hi,";
                let transcript = whisper.clone()
                    .transcribe(reader, language, Some(prompt.to_string())).await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                //println!("Transcribe: {}ms", time.elapsed().as_millis());

                return Ok(Json(transcript));
            },
            _ => {},
        }
    }
    Err((StatusCode::BAD_REQUEST, "No file found".to_string()))
}