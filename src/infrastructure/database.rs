#![allow(missing_docs)]

use juniper::{
  //
  graphql_interface,
  graphql_object,
  graphql_subscription,
  Context,
  DefaultScalarValue,
  FieldError,
  GraphQLEnum,
};
use std::{
  //
  collections::HashMap,
  pin::Pin,
  time::Duration,
};
use super::super::infrastructure::{
  repository::{
  info_rep,
  },
  entitiy:: {
    human::*,
    droid::*,
    episode::*,
    charactor::*
  }
};

/**
 * クエリ定義
 */
pub struct Query;

#[graphql_object(context = Database)]
/// The root query object of the schema
impl Query {
  #[graphql(arguments(id(description = "id of the human")))]
  fn human(database: &Database, id: String) -> Option<&Human> {
    database.get_human(&id)
  }

  #[graphql(arguments(id(description = "id of the droid")))]
  fn droid(database: &Database, id: String) -> Option<&Droid> {
    database.get_droid(&id)
  }

  #[graphql(arguments(
    episode(description = "If omitted, returns the hero of the whole saga. \
                       If provided, returns the hero of that particular episode")
  ))]
  fn hero(database: &Database, episode: Option<Episode>) -> Option<CharacterValue> {
    Some(database.get_hero(episode))
  }

  #[graphql(arguments())]
  fn game_info(database: &Database, id: String) -> Option<Vec<info_rep::GameInfoEntitiy>> {
    database.find_all_game_info()
  }
}


// テーブルの列挙
#[derive(Default, Clone)]
pub struct Database {
  humans: HashMap<String, Human>,
  droids: HashMap<String, Droid>,
}

impl Context for Database {}

impl Database {
  pub fn new() -> Database {
    let mut humans = HashMap::new();
    let mut droids = HashMap::new();

    humans.insert(
      "1000".to_owned(),
      Human::new(
        "1000",
        "Luke Skywalker",
        &["1002", "1003", "2000", "2001"],
        &[Episode::NewHope, Episode::Empire, Episode::Jedi],
        None,
        Some("Tatooine"),
      ),
    );

    droids.insert(
      "2000".to_owned(),
      Droid::new(
        "2000",
        "C-3PO",
        &["1000", "1002", "1003", "2001"],
        &[Episode::NewHope, Episode::Empire, Episode::Jedi],
        None,
        Some("Protocol"),
      ),
    );

    Database { humans, droids }
  }

  pub fn get_hero(&self, episode: Option<Episode>) -> CharacterValue {
    if episode == Some(Episode::Empire) {
      self.get_human("1000").unwrap().clone().into()
    } else {
      self.get_droid("2000").unwrap().clone().into()
    }
  }

  pub fn get_human(&self, id: &str) -> Option<&Human> {
    self.humans.get(id)
  }

  pub fn get_droid(&self, id: &str) -> Option<&Droid> {
    self.droids.get(id)
  }

  pub fn get_character(&self, id: &str) -> Option<CharacterValue> {
    if let Some(h) = self.humans.get(id) {
      Some(h.clone().into())
    } else if let Some(d) = self.droids.get(id) {
      Some(d.clone().into())
    } else {
      None
    }
  }

  pub fn get_friends(&self, c: &dyn Character) -> Vec<CharacterValue> {
    c.friends_ids()
      .iter()
      .flat_map(|id| self.get_character(id))
      .collect()
  }
  pub fn find_all_game_info(&self) -> Option<Vec<info_rep::GameInfoEntitiy>> {
    Some(info_rep::find_all())
  }
}

// ロジックの型だとおもう
struct RandomHuman {
  id: String,
  name: String,
}
#[graphql_object(context = Database)]
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
#[graphql_subscription(context = Database)]
impl Subscription {
  #[graphql(
    description = "A random humanoid creature in the Star Wars universe every 3 seconds. Second result will be an error."
  )]
  async fn random_human(context: &Database) -> RandomHumanStream {
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
        let human = context.get_human(&random_id).unwrap().clone();

        Ok(RandomHuman {
          id: human.id().to_owned(),
          name: human.name().unwrap().to_owned(),
        })
      }
    });

    Box::pin(stream)
  }
}
