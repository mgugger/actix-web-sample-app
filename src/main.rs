#[macro_use]
extern crate serde_derive;

use actix_session::{Session, CookieSession};
use actix_web::{web, App, HttpResponse, HttpServer};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret,
    RedirectUrl, TokenUrl,
};
use std::env;

mod web_auth;

pub struct AppState {
    pub oauth: BasicClient,
    pub api_base_url: String,
}

fn index(session: Session) -> HttpResponse {
    let login = session.get::<String>("login").unwrap();
    let link = if login.is_some() { "logout" } else { "login" };

    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            {} <a href="/{}">{}</a>
        </body>
    </html>"#,
        login.unwrap_or("".to_string()),
        link,
        link
    );

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() {
    HttpServer::new(|| {
        let oauth2_client_id = ClientId::new(
            env::var("OAUTH2_CLIENT_ID")
                .expect("Missing the OAUTH2_CLIENT_ID environment variable."),
        );
        let oauth2_client_secret = ClientSecret::new(
            env::var("OAUTH2_CLIENT_SECRET")
                .expect("Missing the OAUTH2_CLIENT_SECRET environment variable."),
        );
        let oauth2_server =
            env::var("OAUTH2_SERVER").expect("Missing the OAUTH2_SERVER environment variable.");
        
        let auth_url = AuthUrl::new(format!("https://{}/oauth2/v2.0/authorize", oauth2_server))
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(format!("https://{}/oauth2/v2.0/token", oauth2_server))
            .expect("Invalid token endpoint URL");
        
        let api_base_url = "https://graph.microsoft.com/v1.0".to_string();

        // Set up the config for the OAuth2 process.
        let client = BasicClient::new(
            oauth2_client_id,
            Some(oauth2_client_secret),
            auth_url,
            Some(token_url),
        )
        // This example will be running its own server at 127.0.0.1:5000.
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:5000/auth".to_string())
                .expect("Invalid redirect URL"),
        );

        let app_state = web::Data::new(
            AppState {
                oauth: client,
                api_base_url,
            }
        );

        App::new()
            .app_data(app_state)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(web_auth::login))
            .route("/logout", web::get().to(web_auth::logout))
            .route("/auth", web::get().to(web_auth::auth))
    })
    .bind("127.0.0.1:5000")
    .expect("Can not bind to port 5000")
    .run()
    .await
    .unwrap();
}