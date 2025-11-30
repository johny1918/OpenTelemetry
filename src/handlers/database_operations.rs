use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use crate::error::AppError;
use crate::models::user::User;

pub async fn create_user(pool: &mut HashMap<i32, User>, Json(params): Json<User>)
    -> Result<(StatusCode, String), AppError> {

    if params.id.is_nil() || params.email.is_empty() || params.name.is_empty() {
        return Err(AppError::ValidationError("One of the params are empty".to_string()));
    }

    pool.insert(params.id.as_u128() as i32, params)
        .ok_or(AppError::Unknown("Failed to insert".to_string()))?;

    Ok((StatusCode::CREATED, "User created".to_string()))
}

pub async fn get_user(pool: &HashMap<i32, User>, id: i32) -> Result<(StatusCode, String), AppError> {

    if id.is_negative() {
        return Err(AppError::ValidationError("Id must be positive".to_string()));
    }

    let user = pool.get(&id).ok_or(AppError::UserNotFound)?;

    Ok((StatusCode::CREATED, format!("User found: {:?}", user)))

}
