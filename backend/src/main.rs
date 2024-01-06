use askama_axum::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn index() -> Html<String> {
    let template = IndexTemplate { name: "World" };
    Html(template.render().unwrap())
}

async fn click() -> Html<&'static str> {
    Html("<h2>You clicked the button</h2>")
}

#[derive(Deserialize)]
struct Rename {
    name: String,
}

async fn rename(Form(form): Form<Rename>) -> Html<String> {
    Html(form.name)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/click", post(click))
        .route("/rename", post(rename))
        .fallback_service(ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
