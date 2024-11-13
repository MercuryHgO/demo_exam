use sqlx::{types::Uuid, SqlitePool};

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
    pub rating: i32,
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
        rating: i32,
    ) -> Self {
        Partner {
            id: "".into(),
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

}

pub async fn get(db: &SqlitePool, id: Uuid) -> crate::Result<Partner> {
    sqlx::query_as!(Partner, "SELECT * FROM 'public.partners' WHERE id = $1", id)
        .fetch_one(db)
        .await
        .map_err(|e| e.into())
}

pub async fn get_all(db: &SqlitePool) -> crate::Result<Vec<Partner>> {
    sqlx::query_as!(Partner, "SELECT * FROM 'public.partners'")
        .fetch_all(db)
        .await
        .map_err(|e| e.into())
}
