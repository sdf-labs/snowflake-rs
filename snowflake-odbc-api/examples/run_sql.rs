use anyhow::Result;
use arrow::util::pretty::pretty_format_batches;
use clap::Parser;
use snowflake_odbc_api::{QueryResult, SnowflakeCertAuth, SnowflakeOdbcApi};
use std::fs;

extern crate snowflake_odbc_api;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to RSA PEM private key
    #[arg(long)]
    private_key: String,

    /// <account_identifier> in Snowflake format, uppercase
    #[arg(short, long)]
    account_identifier: String,

    /// Database name
    #[arg(short, long)]
    database: String,

    /// Schema name
    #[arg(long)]
    schema: String,

    /// Warehouse
    #[arg(short, long)]
    warehouse: String,

    /// username to whom the private key belongs to
    #[arg(short, long)]
    username: String,

    /// role which user will assume
    #[arg(short, long)]
    role: String,

    /// sql statement to execute and print result from
    #[arg(long)]
    sql: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let pem = fs::read(&args.private_key)?;

    let auth = SnowflakeCertAuth::new(
        &pem,
        &args.username,
        &args.role,
        &args.account_identifier,
        &args.warehouse,
        &args.database,
    )?;

    let api = SnowflakeOdbcApi::new(Box::new(auth), &args.account_identifier)?;
    let res = api.exec(&args.sql).await?;
    match res {
        QueryResult::Arrow(a) => {
            println!("{}", pretty_format_batches(&a).unwrap());
        }
        QueryResult::Empty => {
            println!("Query finished successfully")
        }
    }

    Ok(())
}