use rusqlite::{Connection, Result, params};
use sql_trainer::execute_user_select;
use std::io;

fn main() -> Result<()> {
    let db = std::env::args().nth(1).expect("No db given");
    println!("db: {:?}", db);

    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    connection.execute(
        "INSERT INTO person (id, name, data) VALUES (?1, ?2, ?3)",
        params![0, "paul", "something"],
    )?;

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
