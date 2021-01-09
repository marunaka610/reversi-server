
use juniper::{
  //
  GraphQLEnum,
};

#[derive(GraphQLEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Episode {
  #[graphql(name = "NEW_HOPE")]
  NewHope,
  Empire,
  Jedi,
}