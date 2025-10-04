use std::result::Result;

use std::path::Path;

use rusqlite::Connection;
use rusqlite::types::ValueRef;

use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::{Ollama, error::OllamaError};

use regex::Regex;

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

async fn send_message_to_ollama(message: &str) -> Result<String, ollama_rs::error::OllamaError> {
    let mut ollama = Ollama::default();
    let model = "qwen3:latest".to_string();
    let mut history = vec![];

    let res = ollama
        .send_chat_messages_with_history(
            &mut history,
            ChatMessageRequest::new(model, vec![ChatMessage::user(String::from(message))]),
        )
        .await;

    Ok(res.unwrap().message.content)
}

fn format_output(output: &str) -> String {
    let re = Regex::new(r"(?s)<think>.*?</think>").unwrap();
    re.replace_all(output, "").to_string()
}

pub async fn generate_db(
    topic: &str,
    conn: &Connection,
) -> Result<(), ollama_rs::error::OllamaError> {
    let prompt = format!(
        "Create sql instructions for a sqlite database about {topic}. Please ONLY output sql Instructions. Nothing else. The output gets directly converted into a .sqlite file. Be creative with the topic and create sample data, not just tables. Also be sure to create more lots of diverse entries in the tables so they aren't that empty, but dont repeat yourself. Make sure the syntax is correct and safe and do not use strings in actual data, as this causes parsing problems."
    );
    let mut message = send_message_to_ollama(&prompt).await?;
    message = format_output(&message).to_string();

    println!("{:?}", message);

    conn.execute_batch(&message)
        .expect("Failed to execute insert queries");

    Ok(())
}

pub async fn ask_sql_question(message: &str) -> Result<String, OllamaError> {
    let prompt = String::from(
        "Please answer the following question as an sql-expert in sqlite. Be concise, short and only mention relevant things. Message: ",
    ) + message;
    let output = send_message_to_ollama(&prompt).await?;
    Ok(format_output(&output))
}
