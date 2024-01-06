use askama_axum::Template;
use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Form, Router,
};
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Template)]
#[template(path = "mypage.html")]
struct MyPageTemplate {
    users: Vec<User>,
}

async fn index() -> Html<String> {
    let template = IndexTemplate { name: "World" };
    Html(template.render().unwrap())
}

async fn click() -> Html<&'static str> {
    Html("<h2>You clicked the button</h2>")
}

async fn mypage(Extension(dbpool): Extension<PgPool>) -> Html<String> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM user_account")
        .fetch_all(&dbpool)
        .await
        .unwrap();
    let template = MyPageTemplate { users };
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
    let dbpool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&"postgresql://postgres@localhost/test")
        .await
        .expect("failed to connect to database");

    let app = Router::new()
        .route("/", get(index))
        .route("/click", post(click))
        .route("/rename", post(rename))
        .route("/mypage", get(mypage))
        .layer(Extension(dbpool))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
