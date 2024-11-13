use sqlx::{
    query, query_as,
    sqlite::SqliteQueryResult,
    types::time::Date, SqlitePool,
};
use uuid::Uuid;

pub struct Sale {
    pub product_id: String,
    pub quantity: i64,
    pub sale_date: Date,
    pub partner_id: String,
    pub id: String,
}

impl Sale {
    pub fn new(product_id: String, quantity: i64, sale_date: Date, partner_id: String) -> Self {
        Sale {
            product_id,
            quantity,
            sale_date,
            partner_id,
            id: Uuid::new_v4().to_string(),
        }
    }

    pub async fn create(&self, db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        query!(
            "INSERT INTO sales VALUES ($1,$2,$3,$4,$5)",
            self.product_id,
            self.quantity,
            self.sale_date,
            self.partner_id,
            self.id
        )
        .execute(db)
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete(&self, db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        query!("DELETE FROM sales WHERE id = $1", self.id)
            .execute(db)
            .await
            .map_err(|e| e.into())
    }
}

pub async fn get(db: &SqlitePool, id: String) -> crate::Result<Sale> {
    query_as!(Sale, "SELECT * FROM sales WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}
pub async fn get_all(db: &SqlitePool) -> crate::Result<Vec<Sale>> {
    query_as!(Sale, "SELECT * FROM sales")
        .fetch_all(db)
        .await
        .map_err(|e| e.into())
}
