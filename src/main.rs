use arcgis::{authorize, error, oauth2};
use tracing::{info, trace};

#[tokio::main]
async fn main() -> Result<(), error::ArcError> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    dotenv::dotenv().ok();
    trace!("Environmental variables loaded.");
    let client_id = std::env::var("CLIENT_ID")?;
    let client_secret = std::env::var("CLIENT_SECRET")?;
    let auth_url = std::env::var("OAUTH2")?;
    let token_url = std::env::var("TOKEN")?;
    let redirect_url = std::env::var("REDIRECT")?;
    // let username = std::env::var("USERNAME")?;
    // let password = std::env::var("PASSWORD")?;
    // let host = std::env::var("HOST")?;
    // let url = std::env::var("AUTHORIZE")?;
    // info!("Host: {}", host);

    // let auth_req = authorize::AuthRequest::new(&username, &password, &host);
    // let res = auth_req.authorize(&url).await?;
    let auth = oauth2::AuthReq::new()
        .client_id(&client_id)
        .client_secret(&client_secret)
        .auth_url(&auth_url)
        .token_url(&token_url)
        .redirect_url(&redirect_url)
        .build()?;
    // info!("Auth: {:#?}", auth);
    let res = auth.req().await?;
    Ok(())
}
