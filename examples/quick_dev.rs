use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/hello?name=vale")
		.await?
		.print()
		.await?;

	hc.do_get("/bye/jacob/asper")
		.await?
		.print()
		.await?;

	hc.do_get("/images/ferris_new_years.png")
		.await?
		.print()
		.await?;

	Ok(())
}
