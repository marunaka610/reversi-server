use super::{
  entitiy::ProgressEntitiy,
};

pub trait ProgressDao {
  // 全検索
  fn find_all(&self) -> Vec<ProgressEntitiy>;
  // 1件検索
  fn find_unique(&self, id: i32) -> ProgressEntitiy;
  // 1件挿入
  // fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize;
}

pub trait HaveProgressDao {
  type ProgressDao: ProgressDao;
  fn progress_dao(&self) -> &Self::ProgressDao;
}
