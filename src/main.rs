use std::net::SocketAddr;

use axum::{response::Html, routing::get, serve, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let routes_hello = Router::new().route(
		"/hello",
		get(|| async { Html("<h1>hello from the server</h1>") }),
	);

	let address = SocketAddr::from(([127, 0, 0, 1], 8080));
	let listener = TcpListener::bind(address)
		.await
		.unwrap();
	println!("Listening on http://{address}");

	serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();
}
