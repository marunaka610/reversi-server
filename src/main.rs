extern crate reversi_server;
use actix_cors::Cors;
use actix_web::HttpResponse;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use reversi_server::{
    app::graphql::{graphql, schema, subscriptions},
    config,
};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env().unwrap();

    HttpServer::new(|| {
        App::new()
            .data(schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/subscriptions").route(web::get().to(subscriptions)))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/ise").route(web::get().to(internal_sv_err)))
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}

// エラーコードの検証
#[derive(Deserialize)]
pub struct RequestBody {
    // id: u16,
}
async fn internal_sv_err() -> actix_web::Result<HttpResponse> {
    return Ok(HttpResponse::Unauthorized().finish());
}
