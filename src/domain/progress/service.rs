use super::{
  dto::ProgressDto,
  super::super::infrastructure::repository::progress::dao::{
    ProgressDao,
    HaveProgressDao
  }
};

pub trait ProgressService : HaveProgressDao {

  // 全検索
  fn find_all_progress(&self) -> Vec<ProgressDto> {
    self.progress_dao()
      .find_all()
      .iter()
      .map(|e| ProgressDto::from_entitiy(e))
      .collect()
  }
  // 1件検索
  fn find_unique_progress(&self, id: i32) -> ProgressDto {
  
    let entitiy = self.progress_dao().find_unique(id);
    ProgressDto::from_entitiy(&entitiy)
  }

  // 1件挿入
  // fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize {

  //   self.progress_dao().insert(gameid_param, state_param)
  // }
}

impl<T:HaveProgressDao> ProgressService for T {}

pub trait HaveProgressService {
  type T: ProgressService;
  fn progress_service(&self) -> &Self::T;
}
