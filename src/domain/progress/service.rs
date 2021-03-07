use crate::{
    domain::progress::dto::ProgressDto,
    infrastructure::repository::progress::dao::{HaveProgressDao, ProgressDao},
    util::error::CustomError,
};

pub trait ProgressService: HaveProgressDao {
    // # 全検索
    fn find_all(&self, id: i32) -> Result<Vec<ProgressDto>, CustomError> {
        match self.progress_dao().find_all(id) {
            Ok(entities) => Ok(entities
                .iter()
                .map(|e| ProgressDto::from_entitiy(e))
                .collect()),
            Err(msg) => Err(msg),
        }
    }
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<ProgressDto, CustomError> {
        match self.progress_dao().find_unique(id) {
            Ok(entitiy) => Ok(ProgressDto::from_entitiy(&entitiy)),
            Err(msg) => Err(msg),
        }
    }

    // 1件挿入
    // fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize {

    //   self.progress_dao().insert(gameid_param, state_param)
    // }
}

impl<T: HaveProgressDao> ProgressService for T {}

pub trait HaveProgressService {
    type T: ProgressService;
    fn progress_service(&self) -> &Self::T;
}
