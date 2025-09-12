use crate::pagination::{Paged, Pagination};
use sea_orm::{DbErr, SelectorTrait};

pub trait IntoPaged<S>
where
    S: SelectorTrait,
{
    fn into_paged(
        self,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Paged<S::Item>, DbErr>>;
}
