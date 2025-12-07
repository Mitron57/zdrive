use dotenv::dotenv;
use sqlx::PgPool;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn extract_db_name(database_url: &str) -> anyhow::Result<String> {
    // Parse postgresql://user:pass@host:port/dbname
    let parts: Vec<&str> = database_url.split('/').collect();
    if parts.len() < 4 {
        return Err(anyhow::anyhow!("Invalid DATABASE_URL format"));
    }
    let db_part = parts.last().unwrap();
    // Remove query parameters if any
    let db_name = db_part.split('?').next().unwrap();
    Ok(db_name.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализация логирования
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| {
            error!("DATABASE_URL environment variable is not set");
            anyhow::anyhow!("DATABASE_URL must be set")
        })?;

    // Extract database name from URL
    let db_name = extract_db_name(&database_url)?;
    
    // Connect to postgres database to create target database if needed
    let admin_url = database_url.replace(&format!("/{}", db_name), "/postgres");
    info!("Connecting to postgres database to check/create target database...");
    let admin_pool = PgPool::connect(&admin_url).await
        .map_err(|e| {
            error!("Failed to connect to postgres database: {}", e);
            e
        })?;
    
    // Check if database exists and create if not
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
    .bind(&db_name)
    .fetch_one(&admin_pool)
    .await?;
    
    if !db_exists {
        info!("Database '{}' does not exist, creating...", db_name);
        sqlx::query(&format!(r#"CREATE DATABASE "{}""#, db_name))
            .execute(&admin_pool)
            .await?;
        info!("Database '{}' created successfully", db_name);
    }
    
    drop(admin_pool);

    info!("Starting database migrations...");
    let pool = PgPool::connect(&database_url).await
        .map_err(|e| {
            error!("Failed to connect to database for migrations: {}", e);
            e
        })?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            e
        })?;

    info!("Database migrations completed successfully.");

    Ok(())
}

