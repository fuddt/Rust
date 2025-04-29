use axum::{
    routing::{get, post},
    Router,
    response::{Html, Redirect},
    Form,
    extract::{Extension, Query},
};
use std::net::SocketAddr;
use hyper::Server;
use askama::Template; // Import the derive macro
use serde::Deserialize;

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate;

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate<'a> {
    name: &'a str,
}

#[derive(Deserialize)]
struct GreetForm {
    name: String,
}

#[derive(Deserialize)]
struct ResultQuery {
    name: String,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/greet", post(greet_handler))
        .route("/result", get(result_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> Html<String> {
    let template = FormTemplate;
    Html(template.render().unwrap())
}

async fn greet_handler(Form(data): Form<GreetForm>) -> impl axum::response::IntoResponse {
    Redirect::to(&format!("/result?name={}", data.name))
}

async fn result_handler(Query(query): Query<ResultQuery>) -> Html<String> {
    let template = ResultTemplate { name: &query.name };
    Html(template.render().unwrap())
}