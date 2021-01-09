use super::{
  //
  super::database::Database,
  charactor::*,
  episode::*,
};
use juniper::{graphql_interface, graphql_object};

#[derive(Clone)]
pub struct Droid {
  id: String,
  name: String,
  friend_ids: Vec<String>,
  appears_in: Vec<Episode>,
  secret_backstory: Option<String>,
  primary_function: Option<String>,
}

impl Droid {
  pub fn new(
    id: &str,
    name: &str,
    friend_ids: &[&str],
    appears_in: &[Episode],
    secret_backstory: Option<&str>,
    primary_function: Option<&str>,
  ) -> Self {
    Self {
      id: id.to_owned(),
      name: name.to_owned(),
      friend_ids: friend_ids
        .to_owned()
        .into_iter()
        .map(ToOwned::to_owned)
        .collect(),
      appears_in: appears_in.to_vec(),
      secret_backstory: secret_backstory.map(ToOwned::to_owned),
      primary_function: primary_function.map(ToOwned::to_owned),
    }
  }
}
/// A mechanical creature in the Star Wars universe.
#[graphql_object(context = Database, impl = CharacterValue)]
impl Droid {
  /// The id of the droid
  fn id(&self) -> &str {
    &self.id
  }

  /// The name of the droid
  fn name(&self) -> Option<&str> {
    Some(self.name.as_str())
  }

  /// The friends of the droid
  fn friends(&self, ctx: &Database) -> Vec<CharacterValue> {
    ctx.get_friends(self)
  }

  /// Which movies they appear in
  fn appears_in(&self) -> &[Episode] {
    &self.appears_in
  }

  /// The primary function of the droid
  fn primary_function(&self) -> &Option<String> {
    &self.primary_function
  }
}

// mapper?
#[graphql_interface]
impl Character for Droid {
  fn id(&self) -> &str {
    &self.id
  }

  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn friends(&self, ctx: &Database) -> Vec<CharacterValue> {
    ctx.get_friends(self)
  }

  fn appears_in(&self) -> &[Episode] {
    &self.appears_in
  }

  fn friends_ids(&self) -> &[String] {
    &self.friend_ids
  }
}
