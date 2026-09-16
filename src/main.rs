use askama::Template;
use axum::{Router, extract::Query, response::Html, routing::get};
use serde::Deserialize;

use crate::error::AppError;

mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening to http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[derive(Template)]
#[template(
    ext = "html",
    source = "
<ul>
    {% for domain in domains %}
        <li>{{domain}}</li>
    {% endfor %}
</ul>
"
)]
struct SearchTemplate {
    domains: Vec<String>,
}

async fn search(Query(SearchParams { q }): Query<SearchParams>) -> Result<Html<String>, AppError> {
    Ok(Html(SearchTemplate { domains: vec![q] }.render()?))
}
