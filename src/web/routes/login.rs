use crate::{Error, Result};

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
	Router::new().route("/api/login", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	password: String,
}

async fn login(
	Json(LoginPayload { username, password }): Json<LoginPayload>,
) -> Result<Json<Value>> {
	// TODO: real auth and db logic
	if username != "jim" || password != "123" {
		return Err(Error::LoginFail);
	}

	let body = Json(json!({
		"result": {"success": true}
	}));

	Ok(body)
}
