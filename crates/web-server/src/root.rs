use axum::{Extension, Json};
use db::User;

use crate::error::CustomError;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;
    let users = db::queries::get_users().bind(&client).all().await?;
    Ok(Json(users))
}
