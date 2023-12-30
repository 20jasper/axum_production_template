use std::net::SocketAddr;

use axum::{serve, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod error;
mod web;

pub use self::error::{Error, Result};
use web::routes::{greeting, login};

#[tokio::main]
async fn main() {
	let routes = Router::new()
		.merge(greeting::routes())
		.merge(login::routes())
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
