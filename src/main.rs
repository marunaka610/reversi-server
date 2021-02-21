
extern crate reversi_server;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{
    web, 
    App, 
    middleware,
    HttpServer, 
};
use reversi_server::app::graphql::{
    schema, 
    subscriptions,
    graphql
};
use reversi_server::config_;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config_::Config::from_env().unwrap();
  

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
            .service(web::resource("/subscriptions")
                .route(web::get().to(subscriptions)))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
