#![allow(missing_docs)]
use crate::app::app_context::AppContext;
use juniper::{graphql_object, graphql_subscription, DefaultScalarValue, FieldError};
use std::{pin::Pin, time::Duration};

// ロジックの型だとおもう
struct RandomHuman {
    id: String,
    name: String,
}
#[graphql_object(context = AppContext)]
impl RandomHuman {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
type RandomHumanStream =
    Pin<Box<dyn futures::Stream<Item = Result<RandomHuman, FieldError>> + Send>>;

// 変更通知
pub struct Subscription;
#[graphql_subscription(context = AppContext)]
impl Subscription {
    #[graphql(
        description = "A random humanoid creature in the Star Wars universe every 3 seconds. Second result will be an error."
    )]
    async fn random_human(context: &AppContext) -> RandomHumanStream {
        let mut counter = 0;

        let context = (*context).clone();

        use rand::{rngs::StdRng, Rng, SeedableRng};
        let mut rng = StdRng::from_entropy();

        let stream = tokio::time::interval(Duration::from_secs(3)).map(move |_| {
            counter += 1;

            if counter == 2 {
                Err(FieldError::new(
                    "some field error from handler",
                    Value::Scalar(DefaultScalarValue::String(
                        "some additional string".to_string(),
                    )),
                ))
            } else {
                let random_id = rng.gen_range(1000, 1005).to_string();
                // let human = context.get_human(&random_id).unwrap().clone();
                // let human = Human{id :"a".to_string()};
                Ok(RandomHuman {
                    id: "a".to_string(),
                    name: "a".to_string(),
                })
            }
        });

        Box::pin(stream)
    }
}
