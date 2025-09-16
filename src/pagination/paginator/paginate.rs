use crate::pagination::{IntoPaged, Paged, Paginate, Pagination};
use sea_orm::{
    ConnectionTrait, DbErr, EntityTrait, FromQueryResult, Paginator, PaginatorTrait, Select
    , Selector, SelectorTrait,
};

impl<'db, C, S> Paginate<'db, C> for Selector<S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    type Selector = S::Item;

    async fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> Result<Paged<Self::Selector>, DbErr> {
        let paginator = PaginatorTrait::paginate(self, conn, pagination.page_size);

        let paged = paginator.into_paged(pagination).await?;

        Ok(paged)
    }
}

impl<'db, C, M, E> Paginate<'db, C> for Select<E>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync + 'db,
{
    type Selector = M;

    async fn paginate(
        self,
        conn: &'db C,
        pagination: &Pagination,
    ) -> Result<Paged<Self::Selector>, DbErr> {
        let paginator = PaginatorTrait::paginate(self, conn, pagination.page_size);

        let paged = paginator.into_paged(pagination).await?;

        Ok(paged)
    }
}

impl<'db, C, S> IntoPaged<S> for Paginator<'db, C, S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    async fn into_paged(self, pagination: &Pagination) -> Result<Paged<S::Item>, DbErr> {
        let page_numbers = self.num_items_and_pages().await?;

        let items = self.fetch_page(pagination.page.to_owned()).await?;

        Ok(Paged {
            data: items,
            total: page_numbers.number_of_items,
            page: page_numbers.number_of_pages,
            page_size: pagination.page_size,
        })
    }
}
