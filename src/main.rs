use rusqlite::{params, Connection, Result};
use std::io;

fn main() -> Result<()> {
    let db = std::env::args().nth(1).expect("No db given");
    // let parameter = std::env::args().nth(2).expect("No parameter given");
    println!("pattern: {:?}, path: {:?}", pattern, path);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}
