# Crypto News API Viewer

This is a simple web application built with Rust using the `axum` framework. It fetches the latest cryptocurrency news from multiple sources including NewsData.io and CoinMarketCap APIs and displays it through a clean frontend interface.

## Features

- Search for cryptocurrency news by ticker symbol or name
- Automatic conversion of cryptocurrency symbols to full names
- Aggregates news from multiple sources (NewsData.io and CoinMarketCap)
- Clean, responsive user interface
- Real-time news fetching with API integration

## Technologies Used

- [Rust](https://www.rust-lang.org/) - Core programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework for building the API
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- [Reqwest](https://docs.rs/reqwest) - HTTP client for API requests
- [Serde](https://serde.rs/) - Serialization/deserialization framework
- [NewsData.io API](https://newsdata.io/) - For general cryptocurrency news
- [CoinMarketCap API](https://coinmarketcap.com/api/) - For cryptocurrency specific information

## Project Structure
Blockchain2_1/
├── src/
│   └── main.rs          # Main application code and API endpoints
├── static/
│   ├── main.html        # Frontend HTML template
│   ├── script.js        # Frontend JavaScript
│   └── style.css        # CSS styling
├── target/              # Compiled output (generated)
├── .env                 # Environment variables for API keys
├── .gitignore           # Git ignore file
├── Cargo.lock           # Dependency lock file
├── Cargo.toml           # Project configuration and dependencies
├── LICENCE.md           # License information
└── README.md            # Project documentation
## Getting Started

### Prerequisites

- Rust and Cargo (install from [https://rustup.rs/](https://rustup.rs/))
- API keys for:
    - NewsData.io
    - CoinMarketCap

### Installation

1. Clone the repository
2. Create a `.env` file in the project root with:
   NEWSAPI_KEY=your_newsdata_io_api_key
   COINMARKETCAP_API_KEY=your_coinmarketcap_api_key
3. Run the application:
cargo run
4. Open your browser and navigate to `http://127.0.0.1:8080`

## Usage

Enter a cryptocurrency symbol (e.g., BTC, ETH) or name (e.g., Bitcoin, Ethereum) in the search box to fetch the latest news about that cryptocurrency.

## Built With

- Rust - Backend API and server
- HTML/CSS/JS - Frontend interface
- dotenv - Environment variable management
- chrono - Date/time handling