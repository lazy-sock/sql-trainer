use rusqlite::{params, Connection, Result, Row};
use rusqlite::types::{ValueRef};
use tabled::settings::height::CellHeightIncrease;
use std::io;
use tabled::{
    Tabled, Table,
    settings::{Style, Alignment, object::Columns},
    builder::Builder
};

#[derive(Tabled)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: String,
}

fn execute_user_select(conn: &Connection, query: &str) -> Result<()> {
    let mut stmt = conn.prepare(query)?;
    let col_count = stmt.column_count();

    let col_names: Vec<String> = stmt
        .column_names()
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();

    let mut rows = stmt.query([])?;

    let mut builder = Builder::default();
    builder.push_record(col_names.iter().map(|h| h.clone()));

    
    while let Some(row) = rows.next()? {
        let mut cells = Vec::new();
        for i in 0..col_count {
            let value_ref = row.get_ref(i)?;
            
            let display_value = match value_ref {
                ValueRef::Null => "NULL".to_string(),
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                ValueRef::Blob(b) => format!("[BLOB, len={}]", b.len()),
            };
            cells.push(display_value);
        }
        builder.push_record(cells);
    }

    let mut table = builder.build();
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    println!("{table}");

    Ok(())
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

    execute_user_select(&connection, "SELECT * FROM person;");

    Ok(())
}
