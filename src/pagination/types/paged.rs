use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

impl<T> Paged<T> {
    pub fn map<O, F>(self, mapper: F) -> Paged<O>
    where
        F: FnMut(T) -> O,
    {
        let mapped = self.data.into_iter().map(mapper).collect::<Vec<O>>();

        Paged {
            data: mapped,
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        }
    }
}
