use std::fs;
use std::path::PathBuf;
use sqlx::PgPool;
use dotenv::dotenv;

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
    dotenv().ok();
    
    // Загружаем переменные окружения
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable is not set"))?;

    // Extract database name from URL
    let db_name = extract_db_name(&database_url)?;
    
    // Connect to postgres database to create target database if needed
    let admin_url = database_url.replace(&format!("/{}", db_name), "/postgres");
    println!("Connecting to postgres database to check/create target database...");
    let admin_pool = PgPool::connect(&admin_url).await?;
    
    // Check if database exists and create if not
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
    .bind(&db_name)
    .fetch_one(&admin_pool)
    .await?;
    
    if !db_exists {
        println!("Database '{}' does not exist, creating...", db_name);
        sqlx::query(&format!(r#"CREATE DATABASE "{}""#, db_name))
            .execute(&admin_pool)
            .await?;
        println!("Database '{}' created successfully", db_name);
    }
    
    drop(admin_pool);

    println!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;
    println!("Connected successfully");

    // Получаем путь к папке с миграциями
    let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("migrations");

    if !migrations_dir.exists() {
        return Err(anyhow::anyhow!("Migrations directory not found: {:?}", migrations_dir));
    }

    println!("Reading migrations from: {:?}", migrations_dir);

    // Читаем все SQL файлы из папки миграций
    let mut migration_files: Vec<PathBuf> = fs::read_dir(&migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension() == Some(std::ffi::OsStr::new("sql")) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Сортируем по имени (они должны быть пронумерованы)
    migration_files.sort();

    println!("Found {} migration(s)", migration_files.len());

    // Создаем таблицу для отслеживания выполненных миграций
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version VARCHAR(255) PRIMARY KEY,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Выполняем каждую миграцию
    for migration_file in &migration_files {
        let version = migration_file
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid migration file name"))?;

        // Проверяем, была ли миграция уже выполнена
        let already_applied: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = $1)"
        )
        .bind(version)
        .fetch_one(&pool)
        .await?;

        if already_applied {
            println!("Migration {} already applied, skipping", version);
            continue;
        }

        println!("Applying migration: {}...", version);
        
        // Читаем содержимое файла миграции
        let sql = fs::read_to_string(migration_file)?;

        // Выполняем миграцию в транзакции
        let mut tx = pool.begin().await?;
        
        // Remove comments and split by semicolon
        // Simple approach: remove line comments, then split by semicolon
        let cleaned_sql: String = sql
            .lines()
            .map(|line| {
                // Remove line comments
                if let Some(comment_pos) = line.find("--") {
                    line[..comment_pos].trim_end()
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Split by semicolon and execute each statement
        let statements: Vec<String> = cleaned_sql
            .split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        for statement in statements {
            if !statement.trim().is_empty() {
                sqlx::query(&statement).execute(&mut *tx).await?;
            }
        }

        // Сохраняем информацию о выполненной миграции
        sqlx::query(
            "INSERT INTO schema_migrations (version) VALUES ($1) ON CONFLICT DO NOTHING"
        )
        .bind(version)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        println!("Migration {} applied successfully", version);
    }

    println!("All migrations completed successfully!");
    Ok(())
}

