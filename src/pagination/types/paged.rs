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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paged_map_transforms_data_correctly() {
        #[derive(Debug, PartialEq)]
        struct Item {
            id: u64,
            name: String,
        }

        #[derive(Debug, PartialEq)]
        struct MappedItem {
            mapped_id: u64,
        }

        let paged_items = Paged {
            data: vec![
                Item {
                    id: 1,
                    name: "one".to_string(),
                },
                Item {
                    id: 2,
                    name: "two".to_string(),
                },
            ],
            total: 2,
            page: 1,
            page_size: 10,
        };

        let mapped_paged = paged_items.map(|item| MappedItem {
            mapped_id: item.id * 10,
        });

        assert_eq!(mapped_paged.data.len(), 2);
        assert_eq!(mapped_paged.data[0], MappedItem { mapped_id: 10 });
        assert_eq!(mapped_paged.data[1], MappedItem { mapped_id: 20 });
        assert_eq!(mapped_paged.total, 2);
        assert_eq!(mapped_paged.page, 1);
        assert_eq!(mapped_paged.page_size, 10);
    }
}
