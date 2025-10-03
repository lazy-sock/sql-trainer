use ansi_term::Color::Green;
use clap::Parser;
use rusqlite::{Connection, Result};
use sql_trainer::{ask_sql_question, execute_user_select, file_exists};
use std::fs::remove_file;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sql-trainer")]
#[command(about = "CLI with optional create_db flag")]
struct Cli {
    #[arg(long = "create_db", value_name = "DB_NAME")]
    create_db: Option<String>,

    #[arg(long = "create_db_ai", value_name = "DB_NAME")]
    create_db_ai: Option<String>,

    #[arg(long = "topic", value_name = "PROMPT")]
    topic: Option<String>,

    #[arg(long = "insert_file", value_name = "FILE", requires = "create_db")]
    insert_file: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "DB",
        required_unless_present_any = ["create_db", "create_db_ai"],
    )]
    db: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(dbname) = &cli.create_db {
        let insert_path = cli
            .insert_file
            .as_ref()
            .expect("insert_file required with create_db");
        let full_path = format!("{}.sqlite", dbname);
        if file_exists(&full_path) {
            panic!("db file already exists");
        }

        let conn = Connection::open(&full_path)?;
        println!("Created database at {}", full_path);

        let sql_text = std::fs::read_to_string(insert_path).expect("Failed to read insert file");

        match conn.execute_batch(&sql_text) {
            Ok(()) => {
                println!("Insert queries executed successfully");
            }
            Err(e) => {
                eprintln!("Failed to execute insert queries: {}", e);
                remove_file(&full_path).expect(&format!(
                    "Cleanup Error: Failed to remove file {}",
                    &full_path
                ));
            }
        }

        println!("Inserted from file {}", insert_path.display());

        return Ok(());
    }

    if let Some(dbname) = &cli.create_db_ai {
        let full_path = format!("{}.sqlite", dbname);
        if file_exists(&full_path) {
            panic!("db file already exists");
        }

        let conn = Connection::open(&full_path)?;
        println!("Created database at {}", full_path);

        let topic = cli.topic.expect("Error retrieving topic from arguments");

        //sql_trainer::generate_db(&topic, &conn)
        //    .await
        //    .expect("Could not generate db");

        match sql_trainer::generate_db(&topic, &conn).await {
            Ok(()) => {
                println!("Insert queries executed successfully");
            }
            Err(e) => {
                eprintln!("Falied to execute insert queries: {}", e);
                remove_file(&full_path).expect(&format!(
                    "Cleanup Error: Failed to remove file {}",
                    &full_path
                ))
            }
        }

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
        println!("-----");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Input");

        if input.trim() == "exit" {
            break;
        }

        if input.starts_with("help") {
            println!("Thinking...");
            let output = ask_sql_question(&input)
                .await
                .expect("Error occured while retrieving ollama output");
            println!("Ollama Response: {}", Green.paint(output));
            continue;
        }

        execute_user_select(&connection, &input)?;
    }

    Ok(())
}
