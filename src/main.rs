m#![allow(dead_code)]

mod error;

use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    App,
    HttpResponse,
    HttpServer,
};
use bson::{doc, Bson};
use futures::TryStreamExt;
use mongodb::{options::FindOptions, Client};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
struct KeywordDoc {
    keyword: String,
}

struct AppState {
    client: Client,
}

impl AppState {
    async fn find_keywords(&self) -> Result<Vec<String>> {
        let options = FindOptions::builder()
            .sort(doc! { "keyword": 1 })
            .limit(50)
            .build();

        let cursor = self
            .client
            .database("rust-demo-app")
            .collection("keywords")
            .find(None, options)
            .await?;

        let keywords = cursor
            .and_then(|doc| {
                let doc: KeywordDoc = match bson::from_bson(Bson::Document(doc)) {
                    Ok(doc) => doc,
                    Err(e) => return futures::future::err(e.into()),
                };

                futures::future::ok(doc.keyword)
            })
            .try_collect()
            .await?;

        Ok(keywords)
    }

    async fn insert_keyword_entry(&self, entry: &str) -> Result<()> {
        let keywords = entry.split_whitespace().map(|s| {
            doc! {
                "keyword": s.to_lowercase(),
            }
        });

        self.client
            .database("rust-example-app")
            .collection("keywords")
            .insert_many(keywords, None)
            .await?;

        Ok(())
    }
}

#[actix_web::post("/keyword")]
async fn keyword(data: Data<AppState>, form: Json<KeywordDoc>) -> Result<HttpResponse> {
    data.insert_keyword_entry(&form.keyword).await?;

    Ok(HttpResponse::PermanentRedirect()
        .header("Location", "/")
        .finish())
}

#[actix_web::get("/")]
async fn index(data: Data<AppState>) -> Result<HttpResponse> {
    let content = format!(
        include_str!("../templates/index.html"),
        data.find_keywords().await?.join(", ")
    );

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(content))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let client = Client::with_uri_str(
        std::env::var("MONGODB_URI")
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or("mongodb://localhost:27017"),
    )
    .await?;

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                client: client.clone(),
            })
            .service(index)
            .service(keyword)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}
