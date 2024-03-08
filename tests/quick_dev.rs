#![allow(unused)]

use anyhow::Result;
use serde_json::json;

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

#[tokio::test]
async fn test_static_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/template.html").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_404_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/hjasvgdhasvg").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_api_login_route() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8087")?;

    hc.do_get("/helloPath/Rustest").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username":"userOne",
            "password": "passwordOne"
        }),
    );

    req_login.await?.print().await?;

    hc.do_get("/helloPath/Rustest").await?.print().await?;

    Ok(())
}
