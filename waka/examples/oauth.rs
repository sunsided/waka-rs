//! Signs in with OAuth 2.0 by pasting the code, then reads today's status bar.
//!
//! Register an app at <https://wakatime.com/apps>, add
//! `https://wakatime.com/oauth/test` as an authorized redirect URL, and set
//! `WAKATIME_CLIENT_ID` and `WAKATIME_CLIENT_SECRET` (e.g. in `.env`).

use std::error::Error;
use waka::WakaTimeClientBuilder;
use waka::oauth::{AuthorizeOptions, OAuthClient, Scope};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let client_id = std::env::var("WAKATIME_CLIENT_ID").expect("Missing WAKATIME_CLIENT_ID");
    let client_secret =
        std::env::var("WAKATIME_CLIENT_SECRET").expect("Missing WAKATIME_CLIENT_SECRET");

    let oauth = OAuthClient::new(client_id, client_secret, "https://wakatime.com/oauth/test")?;
    // Use an unguessable value in real applications, and compare it on redirect.
    let state = "example-state";
    let url = oauth.authorize_url(AuthorizeOptions {
        scopes: &[Scope::ReadSummaries],
        state: Some(state),
        ..Default::default()
    });
    println!("Open this URL, approve the app, and paste the code shown:\n\n    {url}\n");

    let mut code = String::new();
    std::io::stdin().read_line(&mut code)?;
    let token = oauth.exchange_code(code.trim()).await?;
    println!("Granted scopes: {:?}", token.scopes());

    let client = WakaTimeClientBuilder::new_with_oauth_token(&token).build()?;
    let status_bar = client.status_bar_today().await?;
    println!("{status_bar:?}");

    // Tokens stay valid for a long time; revoke this one since it was only a demo.
    oauth.revoke(&token.access_token).await?;
    Ok(())
}
