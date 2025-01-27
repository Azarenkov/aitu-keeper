use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::Collection;
use crate::models::grade_overview::grade_overview_model::{GradeOverview, GradesOverview};
use crate::services::grade_overview_service::GradesOverviewRepositoryInterface;

pub struct GradeOverviewRepository {
    collection: Arc<Collection<Document>>,
}

impl GradeOverviewRepository {
    pub fn new(collection: Arc<Collection<Document>>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl GradesOverviewRepositoryInterface for GradeOverviewRepository {
    async fn save(&self, token: &str, grades_overview: &GradesOverview) -> Result<(), Box<dyn Error>> {
        let grades_overview_doc = to_bson(&grades_overview.grades)?;
        self.collection.update_one(doc! {"_id": token}, doc! {
            "$set": {"grades_overview": grades_overview_doc}
        }).await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Vec<GradeOverview>, Box<dyn Error>> {
        todo!()
    }
}