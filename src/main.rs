mod tools;

use ollama_rs::generation::tools::ToolGroup;
use ollama_rs::{Ollama, coordinator::Coordinator, generation::chat::ChatMessage};
use tools::fs_access::{
    create_directory, create_file, gather_directory_context, read_file, write_to_file,
};
use tools::parsers::{
    js_ts::summarize_js_or_ts_code, python::summarize_python_code, rust::summarize_rust_code,
};

use std::error::Error;
use std::io::{self, Write};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let ollama = Ollama::default();
    let history = vec![];
    let tools = ollama_rs::tool_group![
        summarize_rust_code,
        summarize_js_or_ts_code,
        summarize_python_code,
        create_file,
        create_directory,
        write_to_file,
        gather_directory_context,
        read_file
    ];
    let model = "qwen2.5-coder:14b".to_string();

    let mut coordinator = Coordinator::new_with_tools(ollama, model, history, tools);

    let option = select_option()?;
    if option == "chat" {
        chat_loop(&mut coordinator).await?;
    }

    Ok(())
}

/// Displays options and gets user selection
fn select_option() -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("Welcome! What would you like to do?");
    println!("1. Chat with AI");
    print!("Enter the number of your choice: ");
    io::stdout().flush()?; // Flush stdout to ensure prompt is displayed

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    match trimmed {
        "1" => Ok("chat".to_string()),
        _ => {
            println!("Invalid option. Exiting...");
            std::process::exit(1);
        }
    }
}

/// Runs the interactive chat loop
async fn chat_loop<T: ToolGroup>(
    coordinator: &mut Coordinator<Vec<ChatMessage>, T>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("\n🤖 AI: Hello! How can I assist you today?");
    println!("(Type 'exit' to quit the chat)\n");

    loop {
        print!("You: ");
        io::stdout().flush()?; // Ensure user input prompt appears

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim().to_string();

        if user_input.to_lowercase() == "exit" {
            println!("👋 Exiting chat. Have a great day!");
            break;
        }

        let start_time = Instant::now();
        let user_message = ChatMessage::user(user_input);
        let resp = coordinator.chat(vec![user_message]).await?;
        let elapsed_time = start_time.elapsed();

        println!("🤖 AI: {}\n", resp.message.content);
        println!(
            "🕒 Response time: {:.2} seconds\n",
            elapsed_time.as_secs_f64()
        );
    }

    Ok(())
}
