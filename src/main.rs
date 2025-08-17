use axum::{Router, extract::Json, routing::post};
use blue_rust::RawLog;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ingest", post(ingest));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn ingest(Json(payload): Json<Vec<RawLog>>) {
    for log in payload {
        println!("Received log: {:?}\nService: {:?}\n Content: {:?}\n\n", log.msg_id, log.service, log.content);
    }
}
