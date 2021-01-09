mod game;
mod app;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{
    web, 
    App, 
    middleware,
    HttpServer, 
};
use app::graphql::{
    schema, 
    subscriptions,
    graphql
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .bind(format!("{}:{}", "127.0.0.1", 8080))?
    .run()
    .await
}
