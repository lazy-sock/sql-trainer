## Why this project will not be developed further
The problem of this idea is the ollama AI. The AI is capable of outputting correct SQL-Code that relates to the topic provided. However the logic is not very advanced. The database seems correct at first glance, but the inserted data just isn't really representing real data.
For example: Given the task of making a Discord database, the AI creates multiple Servers. In every Server there is exactly one member and one channel. Every member is banned. 

That does not really sound like a realistic scenario, does it? With a better prompt some things can be improved, but it's very questionable if a near-perfect prompt can remove these flaws accross multiple different topics.

Nevertheless the latest version is published to crates.io and is in a usable state.

## sql-trainer

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
