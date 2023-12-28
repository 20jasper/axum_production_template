use std::net::SocketAddr;

use axum::{extract::Query, response::Html, routing::get, serve, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct HelloProps {
	name: Option<String>,
}

async fn hello_handler(Query(HelloProps { name }): Query<HelloProps>) -> Html<String> {
	// prevent extra string allocation with `as_deref`
	let name = name.as_deref().unwrap_or("world");

	Html(format!("<h1>hello {name}</h1>"))
}

#[tokio::main]
async fn main() {
	let routes_hello = Router::new().route("/hello", get(hello_handler));

	let address = SocketAddr::from(([127, 0, 0, 1], 8080));
	let listener = TcpListener::bind(address)
		.await
		.unwrap();
	println!("Listening on http://{address}");

	serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();
}
