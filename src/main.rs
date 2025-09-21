use clap::Parser;
use rusqlite::{Connection, Result};
use sql_trainer::{execute_user_select, file_exists};
use std::io;

#[derive(Parser, Debug)]
#[command(name = "sql-trainer")]
#[command(about = "CLI with optional create_db flag")]
struct Cli {
    #[arg(long = "create_db", value_name = "DB_NAME")]
    create_db: Option<String>,

    #[arg(short, long, value_name = "DB", required_unless_present = "create_db")]
    db: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(dbname) = cli.create_db {
        let full_path = format!("{}.sqlite", dbname);
        if file_exists(&full_path) {
            panic!("db file already exists");
        }

        println!("Created database at {}", full_path);
        return Ok(());
    }

    let db_name = cli.db.expect("db_name must be provided");

    println!("db: {:?}", db_name);

    let full_path = format!("{}.sqlite", db_name);

    if !file_exists(&full_path) {
        panic!("db file does not exist");
    }

    let connection = Connection::open(db_name + ".sqlite")?;

    loop {
        println!("SQL-QUERY:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Input");

        if input.trim() == "exit" {
            break;
        }

        execute_user_select(&connection, &input)?;
    }

    Ok(())
}
