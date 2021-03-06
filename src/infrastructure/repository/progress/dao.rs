use crate::{
    infrastructure::repository::progress::entitiy::ProgressEntitiy, util::error::CustomError,
};

pub trait ProgressDao {
    // 全検索
    fn find_all(&self, id: i32) -> Result<Vec<ProgressEntitiy>, CustomError>;
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<ProgressEntitiy, CustomError>;
    // 1件挿入
    // fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize;
}

pub trait HaveProgressDao {
    type ProgressDao: ProgressDao;
    fn progress_dao(&self) -> &Self::ProgressDao;
}
