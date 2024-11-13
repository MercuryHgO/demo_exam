use sqlx::{sqlite::SqliteQueryResult, types::Uuid, SqlitePool};

#[derive(Clone)]
pub struct Partner {
    pub id: String,
    pub partner_type: String,
    pub partner_name: String,
    pub director: String,
    pub email: String,
    pub phone: String,
    pub legal_address: String,
    pub inn: String,
    pub rating: i64,
}

impl Default for Partner {
    fn default() -> Self {
        Self::new(
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            "".into(),
            0
        )
    }
}

impl PartialEq for Partner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Partner {
    pub fn new(
        partner_type: String,
        partner_name: String,
        director: String,
        email: String,
        phone: String,
        legal_address: String,
        inn: String,
        rating: i64,
    ) -> Self {
        Partner {
            id: Uuid::new_v4().into(),
            partner_type,
            partner_name,
            director,
            email,
            phone,
            legal_address,
            inn,
            rating,
        }
    }

    pub async fn create(&self, db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        sqlx::query!(r#"INSERT INTO partners VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
            self.partner_type,
            self.partner_name,
            self.director,
            self.email,
            self.phone,
            self.legal_address,
            self.inn,
            self.rating,
            self.id,
        )
        .execute(db)
        .await
        .map_err(|e| e.into())
    }
    
    pub async fn delete(&self, db: &SqlitePool) -> crate::Result<SqliteQueryResult> {
        sqlx::query!("DELETE FROM partners WHERE id = $1",
            self.id )
            .execute(db)
            .await
            .map_err(|e| e.into())
    }
}

pub async fn get(db: &SqlitePool, id: String) -> crate::Result<Partner> {
    sqlx::query_as!(Partner, "SELECT * FROM partners WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}

pub async fn get_all(db: &SqlitePool) -> crate::Result<Vec<Partner>> {
    sqlx::query_as!(Partner, "SELECT * FROM partners")
        .fetch_all(db)
        .await
        .map_err(|e| e.into())
}

