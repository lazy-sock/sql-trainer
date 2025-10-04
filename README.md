A CLI for learning SQL in a playground environment. Uses AI to generate Databases.

## Installation
You need to have Rust and Cargo installed.

```cargo install sql-trainer```

## Generate new Database
First install ollama and the model `qwen3:latest`. Then serve the AI with `ollama serve`.

After that you can run the following command with a topic you want. Keep in mind that the topic cannot contain spaces.

`sql-trainer --create_db_ai db_name --topic github`

## Usage
`sql-trainer --db github`

Then just run your SQL Select Queries! You can exit the program with `exit` and ask AI for help with `help <question>` (you have to host it yourself with `ollama serve`).
