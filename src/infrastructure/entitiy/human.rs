use super::{
  //
  super::database::Database,
  episode::*,
  charactor::*
};
use juniper::{graphql_interface, graphql_object};

#[derive(Clone)]
pub struct Human {
  id: String,
  name: String,
  friend_ids: Vec<String>,
  appears_in: Vec<Episode>,
  secret_backstory: Option<String>,
  home_planet: Option<String>,
}

impl Human {
  pub fn new(
    id: &str,
    name: &str,
    friend_ids: &[&str],
    appears_in: &[Episode],
    secret_backstory: Option<&str>,
    home_planet: Option<&str>,
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
      home_planet: home_planet.map(|p| p.to_owned()),
    }
  }
}

/// A humanoid creature in the Star Wars universe.
#[graphql_object(context = Database, impl = CharacterValue)]
impl Human {
  /// The id of the human
  fn id(&self) -> &str {
    &self.id
  }

  /// The name of the human
  fn name(&self) -> Option<&str> {
    Some(self.name.as_str())
  }

  /// The friends of the human
  fn friends(&self, ctx: &Database) -> Vec<CharacterValue> {
    ctx.get_friends(self)
  }

  /// Which movies they appear in
  fn appears_in(&self) -> &[Episode] {
    &self.appears_in
  }

  /// The home planet of the human
  fn home_planet(&self) -> &Option<String> {
    &self.home_planet
  }
}

#[graphql_interface]
impl Character for Human {
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
