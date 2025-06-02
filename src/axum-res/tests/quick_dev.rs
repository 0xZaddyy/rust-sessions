#![allow(unused)]

use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("https://localhost/8080")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}