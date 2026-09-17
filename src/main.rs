use crate::{error::AppError, tlds::TLDS};
use anyhow::Context;
use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use hickory_resolver::TokioResolver;
use serde::Deserialize;

mod error;
mod tlds;

const MAX_SLD_LEN: usize = 63;

#[derive(Clone)]
struct Ctx {
    hickory: TokioResolver,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search))
        .route("/lookup", get(lookup))
        .with_state(Ctx {
            hickory: TokioResolver::builder_tokio()?.build()?,
        });

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
    source = r#"
<ul>
    {% for domain in domains %}
        <li>
            <span>{{domain}}</span>
            <div
                hx-get="http://127.0.0.1:8080/lookup?domain={{domain}}"
                hx-trigger="load"
                hx-swap="outerHTML"
            ></div>
        </li>
    {% endfor %}
</ul>
"#
)]
struct SearchTemplate {
    domains: Vec<String>,
}

async fn search(Query(SearchParams { q }): Query<SearchParams>) -> Result<Html<String>, AppError> {
    let q = q.trim().to_lowercase();
    if q.is_empty() {
        return Ok(Html(String::new()));
    }
    let domain = get_domain(&q)?;
    let mut domains = vec![domain.clone()];
    let sld = domain.split('.').next().context("failed to get sld")?;
    if sld.len() > MAX_SLD_LEN {
        return Err(anyhow::anyhow!("sld cannot be longer than {MAX_SLD_LEN} characters").into());
    }
    for variant in devowel(sld) {
        for i in (1..variant.len() - 1).rev() {
            let (new_sld, new_tld) = variant.split_at(i);
            if TLDS.contains(new_tld) {
                domains.push(format!("{new_sld}.{new_tld}"));
            }
        }
    }
    Ok(Html(SearchTemplate { domains }.render()?))
}

fn get_domain(q: &str) -> anyhow::Result<String> {
    let domain = url::Url::parse(q)
        .ok()
        .and_then(|url| url.domain().map(str::to_owned))
        .unwrap_or_else(|| q.to_owned());
    let sanitized: String = domain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.')
        .collect();
    let trimmed = sanitized.trim_matches('.');
    anyhow::ensure!(
        !trimmed.is_empty(),
        "domain must contain an ascii alphanumeric character"
    );
    let parts: Vec<&str> = trimmed.split('.').filter(|p| !p.is_empty()).collect();
    Ok(parts
        .windows(2)
        .find(|w| TLDS.contains(w[1]))
        .map(|w| format!("{}.{}", w[0], w[1]))
        .unwrap_or_else(|| format!("{}.com", parts[0])))
}

fn devowel(word: &str) -> Vec<String> {
    let mut res = vec![word.to_owned()];
    let mut curr = word.to_owned();
    while curr.len() > 1 {
        let Some(i) = curr.rfind(['a', 'e', 'i', 'o', 'u']) else {
            break;
        };
        curr.remove(i);
        res.push(curr.clone());
    }
    res
}

#[derive(Deserialize)]
struct LookupParams {
    domain: String,
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<a href="https://www.dynadot.com/domain/search?rscreg=shorter&domain={{domain}}">
    {% if available %}
        continue
    {% else %}
        lookup
    {% endif %}
</a>
"#
)]
struct LookupTemplate {
    available: bool,
    domain: String,
}

async fn lookup(
    Query(LookupParams { domain }): Query<LookupParams>,
    State(Ctx { hickory }): State<Ctx>,
) -> Result<Html<String>, AppError> {
    let sld = domain.split('.').next().context("failed to get sld")?;
    let available = sld.len() > 1
        && hickory
            .ns_lookup(format!("{domain}."))
            .await
            .is_err_and(|e| e.is_nx_domain());
    Ok(Html(LookupTemplate { available, domain }.render()?))
}
