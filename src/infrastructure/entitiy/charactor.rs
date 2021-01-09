use super::{
  //
  super::database::Database,
  droid::*,
  episode::*,
  human::*,
};
use juniper::graphql_interface;

#[graphql_interface(for = [Human, Droid], context = Database)]
/// A character in the Star Wars Trilogy
pub trait Character {
  /// The id of the character
  fn id(&self) -> &str;

  /// The name of the character
  fn name(&self) -> Option<&str>;

  /// The friends of the character
  fn friends(&self, ctx: &Database) -> Vec<CharacterValue>;

  /// Which movies they appear in
  fn appears_in(&self) -> &[Episode];

  #[graphql(ignore)]
  fn friends_ids(&self) -> &[String];
}
