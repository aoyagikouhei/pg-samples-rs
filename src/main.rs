mod logs;
mod pg_type;
mod pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logs::prepare_log("pg");
    let pg_url =
        std::env::var("PG_URL").unwrap_or("postgres://user:pass@localhost:5432/web".to_owned());
    let pg_pool = pool::get_postgres_pool(&pg_url)?;
    let pg_client = pg_pool.get().await?;
    pg_type::execute(&pg_client).await?;

    Ok(())
}
