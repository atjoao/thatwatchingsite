use axum::{
    routing::{get, post}, Router,
};

mod structs;

mod sources {
    pub mod nyaa_si;
}

// parse where the request wants needs to go and return it
// [ /search, /t?q=name&source=, /add?hash=, ] 

#[tokio::main]
async fn main(){

    let app = Router::new()
        .route("/search", get(search))
        .route("/t", get(search))
        .route("/add", get(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search() -> &'static str {
    "Hello, World!"
}