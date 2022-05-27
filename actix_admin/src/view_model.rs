use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::{Serialize};

use crate::ActixAdminModel;
use crate::ActixAdminField;

#[async_trait(?Send)]
pub trait ActixAdminViewModelTrait {
    async fn list(
        db: &DatabaseConnection,
        page: usize,
        entities_per_page: usize,
    ) -> Vec<ActixAdminModel>;
    async fn create_entity(db: &DatabaseConnection, model: ActixAdminModel) -> ActixAdminModel;
    fn get_entity_name() -> String;
}

#[derive(Clone, Debug, Serialize)]
pub struct ActixAdminViewModel {
    pub entity_name: String,
    pub fields: Vec<(String, ActixAdminField)>,
}