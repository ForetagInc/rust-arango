//! An example to use reqwest with rustls.
//!
//! 1. use vanilla arangors without any http client implementation by disabling
//! `reqwest_async`, `reqwest_blocking` and `surf_async` on arangors.
//! 2. implement custom reqwest client and enable `rustls` feature gate like in `src/client.rs`.
//! 3. use custom client with `rust_arango::GenericConnection`.
mod client;

use rust_arango::GenericConnection;

use self::client::ReqwestClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    const URL: &str = "http://localhost:8529";
    // use custom Reqwest Client
    let conn =
        GenericConnection::<ReqwestClient>::establish_jwt(URL, "username", "password").await?;
    let db = conn.db("test_db").await?;
    let info = db.info().await?;
    println!("{:?}", info);

    Ok(())
}
