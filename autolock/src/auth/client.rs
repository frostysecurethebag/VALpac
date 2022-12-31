use reqwest::{Client};
pub async fn create_client() -> reqwest::Result<Client> {

    let client = Client::builder()
        .user_agent("RiotClient/60.0.6.4770705.4749685 rso-auth (Windows;10;;Professional, x64)")
        .cookie_store(true)
        // .use_preconfigured_tls(context)
        .build()
        .unwrap();

    Ok(client)
}