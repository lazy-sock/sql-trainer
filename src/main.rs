use rusqlite::{params, Connection, Result};
use std::io;
use tabled::{
    Tabled, Table, assert::assert_table,
    settings::{Style, Alignment, object::Columns},
};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let db = std::env::args().nth(1).expect("No db given");
    println!("db: {:?}", db);
    
    let mut input = String::new();
//    io::stdin().read_line(&mut input)?;

    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;

    let binary_data = vec![1u8, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1];

    // Correctly insert the BLOB data using a placeholder
    connection.execute(
        "INSERT INTO person (id, name, data) VALUES (?1, ?2, ?3)",
        params![0, "paul", binary_data],
    )?;    let mut data = connection.prepare("SELECT * FROM person")?;

    let person_iter = data.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("{:?}", person?);
    }

    let mut table = Table::new(person_iter);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    assert_table!(table, "");

    Ok(())
}
