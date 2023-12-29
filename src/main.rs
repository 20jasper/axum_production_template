use std::net::SocketAddr;

use axum::{
	extract::{Path, Query},
	response::Html,
	routing::get,
	serve, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct HelloParams {
	name: Option<String>,
}

async fn hello_handler(Query(HelloParams { name }): Query<HelloParams>) -> Html<String> {
	// prevent extra string allocation with `as_deref`
	let name = name.as_deref().unwrap_or("world");

	Html(format!("<h1>hello {name}</h1>"))
}

#[derive(Deserialize)]
struct ByeParams {
	name: String,
	last_name: String,
}

async fn bye_handler(Path(ByeParams { name, last_name }): Path<ByeParams>) -> Html<String> {
	Html(format!("<h1>bye {name} {last_name}</h1>"))
}

fn greetings_routes() -> Router {
	Router::new()
		.route("/hello", get(hello_handler))
		.route("/bye/:name/:last_name", get(bye_handler))
}

mod error;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
	let routes = Router::new()
		.merge(greetings_routes())
		.fallback_service(ServeDir::new("public/"));

	let address = SocketAddr::from(([127, 0, 0, 1], 8080));
	let listener = TcpListener::bind(address)
		.await
		.unwrap();
	println!("Listening on http://{address}");

	serve(listener, routes.into_make_service())
		.await
		.unwrap();
}
