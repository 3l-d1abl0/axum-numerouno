#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn test_normal_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_query_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/helloParam?name=Rustest").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_path_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/helloPath/Rustest").await?.print().await?;

    Ok(())
}
