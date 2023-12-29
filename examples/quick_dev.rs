use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_post("/api/login", json!({"username": "jim", "password": "123"}))
		.await?
		.print()
		.await?;

	Ok(())
}
