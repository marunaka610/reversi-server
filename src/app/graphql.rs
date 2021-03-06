use crate::app::{
    app_context::AppContext, mutation::Mutation, query::Query, subscription::Subscription,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::RootNode;
use juniper_actix::{graphql_handler, subscriptions::subscriptions_handler};
use juniper_graphql_ws::ConnectionConfig;
use std::time::Duration;

// # Subscription
pub async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = AppContext::new();
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(context);
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}

// # スキーマ定義
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;
pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

// # GraphQLメソッド
pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = AppContext::new();
    let r = graphql_handler(&schema, &context, req, payload).await;
    r
}
