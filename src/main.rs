use axum::{response::Html, routing::get, Router};
use ct2rs::{Whisper, Config, WhisperOptions};
use hound::WavReader;
use std::path::Path;
use anyhow::Result;
use std::time::Instant;
use tokio::task;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config {
        device: ct2rs::Device::CUDA,
        //num_threads_per_replica: 8,
        compute_type: ct2rs::ComputeType::FLOAT16,
        //tensor_parallel: true,
        ..Default::default()
    };
    let whisper = Arc::new(Whisper::new("./models/turbo", config).unwrap());

    let samples = Arc::new(read_audio("./audio/0007_en-us.wav", whisper.sampling_rate()).unwrap());

    print!("Running generate function...\n");
    let start = Instant::now();
    let options = WhisperOptions {
        beam_size: 1,
        ..Default::default()
    };
    let res = whisper.generate(&samples, Some("en"), false, &options).unwrap();
    for r in res {
        println!("{}", r);
    }
    let duration = start.elapsed();
    println!("generate function took: {:?}", duration);

    let handles: Vec<_> = (0..1).map(|_| {
        let whisper = whisper.clone();
        let samples = samples.clone();
        task::spawn(async move {
            print!("Running generate function...\n");
            let start = Instant::now();
            let res = whisper.generate(&samples, None, false, &Default::default()).unwrap();
            for r in res {
                println!("{}", r);
            }
            let duration = start.elapsed();
            println!("generate function took: {:?}", duration);
        })
    }).collect();

    for handle in handles {
        handle.await.unwrap();
    }

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

fn read_audio<T: AsRef<Path>>(path: T, sample_rate: usize) -> Result<Vec<f32>> {
    // Should use a better resampling algorithm.
    fn resample(samples: Vec<f32>, src_rate: usize, target_rate: usize) -> Vec<f32> {
        samples
            .into_iter()
            .step_by(src_rate / target_rate)
            .collect()
    }

    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();

    let max = 2_i32.pow((spec.bits_per_sample - 1) as u32) as f32;
    let samples = reader
        .samples::<i32>()
        .map(|s| s.unwrap() as f32 / max)
        .collect::<Vec<f32>>();

    if spec.channels == 1 {
        return Ok(resample(samples, spec.sample_rate as usize, sample_rate));
    }

    let mut mono = vec![];
    for chunk in samples.chunks(2) {
        if chunk.len() == 2 {
            mono.push((chunk[0] + chunk[1]) / 2.);
        }
    }

    Ok(resample(mono, spec.sample_rate as usize, sample_rate))
}