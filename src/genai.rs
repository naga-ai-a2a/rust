use std::{env, process};

use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Collect arguments into a Vector
    let args: Vec<String> = env::args().collect();

    // usage
    if args.len() != 2 {
        println!("\n\tneed arg: <user prompt>");
        // return Err(Box::from("need arg: <user prompt>"));
        process::exit(1);
    }

    // 1. Create a common genai client
    let client = Client::default();

    // 2. Build a chat request
    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("You are a helpful assistant."),
        // ChatMessage::user("What is Rust in 10 words?"),
        // ChatMessage::user("What is .await? in Rust?"),
        ChatMessage::user(&args[1]),
    ]);

    // 3. Execute the request using a specific model
    // The library automatically maps the model name to the correct provider
    let model = "gpt-4o-mini"; 
    let chat_res = client.exec_chat(model, chat_req, None).await?;

    // 4. Extract and Pretty-Print the JSON response
    let texts = chat_res.texts();

    // Join with a separator
    let s: String = texts.join(" ");
    println!("\n{}\n", s);

    Ok(())
}
