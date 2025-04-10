use axum::{extract::Query, response::{Json, Html}, routing::get, Router};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;
use chrono::Utc;
use std::fmt::Debug;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsArticle {
    pub title: String,
    pub url: String,
    pub source: String,
    pub date: String,
}

#[derive(Deserialize)]
struct NewsQuery {
    query: String,
}

fn resolve_coin_name(symbol: &str) -> &str {
    match symbol.to_uppercase().as_str() {
        "BTC" => "Bitcoin",
        "ETH" => "Ethereum",
        "BNB" => "Binance Coin",
        "ADA" => "Cardano",
        "DOGE" => "Dogecoin",
        "XRP" => "Ripple",
        "SOL" => "Solana",
        "DOT" => "Polkadot",
        "AVAX" => "Avalanche",
        _ => symbol,
    }
}

async fn fetch_newsapi(query: &str) -> Result<Vec<NewsArticle>, reqwest::Error> {
    let api_key = env::var("NEWS_KEY").expect("NEWS_KEY not set");
    let search_term = resolve_coin_name(query);

    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en",
        api_key,
        search_term.to_lowercase()
    );

    let response = Client::new().get(&url).send().await?;

    let json: serde_json::Value = response.json().await?;
    let mut news = Vec::new();

    if let Some(results) = json["results"].as_array() {
        for a in results.iter().take(5) {
            news.push(NewsArticle {
                title: a["title"].as_str().unwrap_or("").to_string(),
                url: a["link"].as_str().unwrap_or("").to_string(),
                source: a["source_id"].as_str().unwrap_or("NewsData.io").to_string(),
                date: a["pubDate"].as_str().unwrap_or("").to_string(),
            });
        }
    }
    Ok(news)
}

async fn fetch_coinmarketcap(query: &str) -> Result<Vec<NewsArticle>, reqwest::Error> {
    let api_key = env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set");
    let client = Client::new();

    let response = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info")
        .query(&[("symbol", query)])
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let mut news = Vec::new();

    if let Some(data) = json["data"][query].as_object() {
        let _desc = data["description"].as_str().unwrap_or("");
        if let Some(site_url) = data["urls"]["website"][0].as_str() {
            news.push(NewsArticle {
                title: format!("Overview of {}", query),
                url: site_url.to_string(),
                source: "CoinMarketCap".to_string(),
                date: Utc::now().to_rfc3339(),
            });
        }
    }

    Ok(news)
}

async fn fetch_news(Query(params): Query<NewsQuery>) -> Json<Vec<NewsArticle>> {
    let query = params.query.clone();

    let (newsapi_result, cmc_result) = tokio::join!(
        fetch_newsapi(&query),
        fetch_coinmarketcap(&query)
    );

    let mut combined = Vec::new();

    // Обработка результатов из NewsAPI
    if let Ok(newsapi) = newsapi_result {
        combined.extend(newsapi);
    } else {
        eprintln!("Error fetching NewsAPI for query: {}", query);
    }

    // Обработка результатов из CoinMarketCap
    if let Ok(cmc) = cmc_result {
        combined.extend(cmc);
    } else {
        eprintln!("Error fetching CoinMarketCap for query: {}", query);
    }

    Json(combined)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/news", get(fetch_news))
        .route("/", get(|| async {
            Html(include_str!("../static/main.html"))
        }))
        .route("/style.css", get(|| async {
            (
                [("Content-Type", "text/css")],
                include_str!("../static/style.css"),
            )
        }))
        .route("/script.js", get(|| async {
            Html(include_str!("../static/script.js"))
        }));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("🚀 Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app
    )
        .await
        .unwrap();
}