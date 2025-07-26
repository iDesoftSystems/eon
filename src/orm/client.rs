use crate::command::PersistenceClient;

impl PersistenceClient for sea_orm::DatabaseConnection {}
impl PersistenceClient for sea_orm::DatabaseTransaction {}
