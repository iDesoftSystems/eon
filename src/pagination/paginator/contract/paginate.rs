use crate::pagination::{Paged, Pagination};
use sea_orm::{ConnectionTrait, DbErr};

pub trait Paginate<'db, C>
where
    C: ConnectionTrait,
{
    type Selector;

    fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Paged<Self::Selector>, DbErr>>;
}
