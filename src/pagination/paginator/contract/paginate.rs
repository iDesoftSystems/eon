use crate::pagination::{Paged, Pagination};
use sea_orm::{ConnectionTrait, DbErr, SelectorTrait};

pub trait Paginate<'db, C, S>
where
    C: ConnectionTrait,
    S: SelectorTrait,
{
    fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Paged<S::Item>, DbErr>>;
}
