use axum::{Router, routing::get, response::{IntoResponse, Response}};
use std::net::SocketAddr;

async fn download_csv() -> Response {
    let csv_content = "name,age\nAlice,30\nBob,25\n";
    let mut response = csv_content.to_owned().into_response();
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        "text/csv".parse().unwrap(),
    );
    response.headers_mut().insert(
        axum::http::header::CONTENT_DISPOSITION,
        "attachment; filename=\"sample.csv\"".parse().unwrap(),
    );
    response
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/download", get(download_csv));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}