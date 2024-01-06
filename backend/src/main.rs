use askama_axum::Template;
use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
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

#[derive(Template)]
#[template(path = "mypage.html")]
struct MyPageTemplate {}

async fn index() -> Html<String> {
    let template = IndexTemplate { name: "World" };
    Html(template.render().unwrap())
}

async fn click() -> Html<&'static str> {
    Html("<h2>You clicked the button</h2>")
}

async fn mypage() -> Html<String> {
    let template = MyPageTemplate {};
    Html(template.render().unwrap())
}

#[derive(Deserialize)]
struct Rename {
    name: String,
}

async fn rename(Form(form): Form<Rename>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "HX-Trigger-After-Swap",
        r#"{ "namechanged": { "message": "Name successfully changed!" } }"#
            .parse()
            .unwrap(),
    );
    (headers, Html(form.name))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/click", post(click))
        .route("/rename", post(rename))
        .route("/mypage", get(mypage))
        .fallback_service(ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
