use sqlx::{query, query_as, sqlite::SqliteQueryResult, SqlitePool};

use crate::Result;

#[derive(Clone)]
pub struct ProductType {
    pub product_type: String,
    pub coefficient: f64
}

impl PartialEq for ProductType {
    fn eq(&self, other: &Self) -> bool {
        self.product_type == other.product_type
    }
}

impl Default for ProductType {
    fn default() -> Self {
        ProductType::new("".to_string(), 0.0)
    }
}

impl ProductType {
    pub fn new(product_type: String, coefficient: f64) -> Self {
        ProductType { product_type, coefficient }
    }

    pub async fn create(&self, db: &SqlitePool) -> Result<SqliteQueryResult> {
        query!("INSERT INTO product_types VALUES ($1,$2)",
            self.product_type,
            self.coefficient
        )
            .execute(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete(&self, db: &SqlitePool) -> Result<SqliteQueryResult> {
        query(&format!("DELETE FROM product_types WHERE product_type = '{}'",self.product_type))
            .execute(db)
            .await
            .map_err(|e| e.into())
    }
}

pub async fn get(db: &SqlitePool, product_type: String) -> Result<ProductType> {
    query_as!(ProductType,"SELECT * FROM product_types WHERE product_type = $1",product_type)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}

pub async fn get_all(db: &SqlitePool) -> Result<Vec<ProductType>> {
    query_as!(ProductType,"SELECT * FROM product_types")
        .fetch_all(db)
        .await
        .map_err(|e| e.into())
}
