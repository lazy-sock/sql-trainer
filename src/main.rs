use rusqlite::{Connection, Result};
use sql_trainer::{execute_user_select, file_exists};
use std::io;

fn main() -> Result<()> {
    let db = std::env::args().nth(1).expect("No db given");
    println!("db: {:?}", db);

    let full_path = format!("{}.sqlite", db);

    if !file_exists(&full_path) {
        panic!("db file does not exist");
    }

    let connection = Connection::open(db + ".sqlite")?;

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
