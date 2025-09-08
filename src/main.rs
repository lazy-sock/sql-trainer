use rusqlite::{params, Connection, Result};
use std::io;
use tabled::{
    Tabled, Table,
    settings::{Style, Alignment, object::Columns},
};

#[derive(Tabled)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: String,
}

fn main() -> Result<()> {
    let db = std::env::args().nth(1).expect("No db given");
    println!("db: {:?}", db);
    
    println!("SQL-QUERY:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read Input");
    println!("SQL-QUERY: {input}");

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

    let mut table = Table::new(vec![Person{id: 1, name: "adsf".to_string(), data: "adfs".to_string()}]);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    println!("{table}");

    Ok(())
}
