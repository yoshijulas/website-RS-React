use crate::errors::AppError;
use sqlx::PgPool;

pub async fn log_activity(pool: &PgPool, user_id: i32, activity: String) -> Result<(), AppError> {
    sqlx::query!(
        "
        INSERT INTO activity_logs (user_id, user_action) 
        VALUES ($1, $2)
        ",
        user_id,
        activity,
    )
    .execute(pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("An unexpected error occurred {err}")))?;

    Ok(())
}
