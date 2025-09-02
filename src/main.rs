use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;

use axum::{
	extract::Json,
	routing::post,
	Router, Extension,
};
use chrono::Local;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawLog {
	msg_id: String,
	service: String,
	content: String,
}

#[tokio::main]
async fn main() {
	let logger = Logger::new("./logs");

	let app = Router::new()
		.route("/ingest", post(ingest))
		.layer(Extension(logger));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn ingest(
	Extension(logger): Extension<Logger>,
	Json(payload): Json<Vec<RawLog>>,
) {
	for log in payload {
		logger.info(&format!("{}\n{}\n{}", log.msg_id, log.service, log.content));
	}
}

#[derive(Clone)]
struct Logger {
	log_dir: String,
}

impl Logger {
	pub fn new(log_dir: &str) -> Self {
		if !Path::new(log_dir).exists() {
			create_dir_all(log_dir).expect("Failed to create log directory");
		}
		Logger {
			log_dir: log_dir.to_string(),
		}
	}

	pub fn info(&self, message: &str) {
		let log_file = format!("{}/out.log", self.log_dir);
		let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
		let log_line = format!("INFO:: {}: {}", timestamp, message);

		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open(&log_file)
			.expect("Failed to open log file");
		writeln!(file, "{}", log_line).expect("Failed to write log");
	}
}
