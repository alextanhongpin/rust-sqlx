use std::env;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
struct Job { name: String, id: Uuid }

#[async_std::main]
async fn main() -> Result<(), sqlx::Error>{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("got {}", row.0);
    assert_eq!(row.0, 150);

    let job  = sqlx::query_as::<_,Job>("SELECT * FROM job")
        .fetch_one(&pool).await?;
    println!("got job: {:?}", job);

    Ok(())
}

