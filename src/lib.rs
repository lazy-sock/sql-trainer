use std::result::Result;

use std::path::Path;

use rusqlite::Connection;
use rusqlite::types::ValueRef;

use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

pub fn file_exists(filename: &str) -> bool {
    let path = Path::new(filename);

    path.exists()
}

pub fn execute_user_select(conn: &Connection, query: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(query)?;
    let col_count = stmt.column_count();

    let col_names: Vec<String> = stmt
        .column_names()
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();

    let mut rows = stmt.query([])?;

    let mut builder = Builder::default();
    builder.push_record(col_names.iter().cloned());

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

pub fn create_sqlite_file(
    filename: &str,
    sql_statements: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(filename)?;

    for i in sql_statements {
        conn.execute(i, [])?;
    }

    Ok(())
}
