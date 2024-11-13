use sqlx::{query, query_as, sqlite::SqliteQueryResult, SqlitePool};
use uuid::Uuid;

#[derive(Clone,Debug)]
pub struct Product {
    pub product_type: String,
    pub product_name: String,
    pub article_number: String,
    pub minimum_cost: i64,
    pub id: String
}

impl Default for Product {
    fn default() -> Self {
        Product::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            0
        )
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Product {
    pub fn new(
        product_type: String,
        product_name: String,
        article_number: String,
        minimum_cost: i64,
    ) -> Self {
        Product {
            product_type,
            product_name,
            article_number,
            minimum_cost,
            id: Uuid::new_v4().to_string()
        }
    }

    pub async fn create(&self,db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        query(
            &format!("INSERT INTO products VALUES ('{}','{}','{}','{}','{}')",
                self.product_type,
                self.product_name,
                self.article_number,
                self.minimum_cost,
                self.id
            )
        )
            .execute(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete(&self, db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        query(&format!("DELETE FROM products WHERE id = '{}'",self.id))
            .execute(db)
            .await
            .map_err(|e| e.into())
    }

}

pub async fn get(db: &SqlitePool, id: String) -> crate::Result<Product> {
    query_as!(Product,"SELECT * FROM products WHERE id = $1",id)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}

pub async fn get_all(db: &SqlitePool) -> crate::Result<Vec<Product>> {
    query_as!(Product,"SELECT * FROM products")
        .fetch_all(db)
        .await
        .map_err(|e| e.into())
}
