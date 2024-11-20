use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tokio;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug)]
enum AiProvider {
    OpenAI,
    Ollama,
}

struct ChatApp {
    client: Client,
    history: Vec<Message>,
    provider: AiProvider,
    api_key: String,
    model: String,
}

impl ChatApp {
    fn new(provider: AiProvider, api_key: String, model: String) -> Self {
        ChatApp {
            client: Client::new(),
            history: Vec::new(),
            provider,
            api_key,
            model,
        }
    }

    async fn send_message(&mut self, content: String) -> Result<String, Box<dyn std::error::Error>> {
        self.history.push(Message {
            role: "user".to_string(),
            content: content.clone(),
        });

        let response = match self.provider {
            AiProvider::OpenAI => {
                let request = ChatRequest {
                    model: self.model.clone(),
                    messages: self.history.clone(),
                };

                self.client
                    .post("https://api.openai.com/v1/chat/completions")
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .json(&request)
                    .send()
                    .await?
                    .json::<ChatResponse>()
                    .await?
            }
            AiProvider::Ollama => {
                let request = ChatRequest {
                    model: self.model.clone(),
                    messages: self.history.clone(),
                };

                self.client
                    .post("http://localhost:11434/api/chat")
                    .json(&request)
                    .send()
                    .await?
                    .json::<ChatResponse>()
                    .await?
            }
        };

        let assistant_message = response.choices[0].message.clone();
        self.history.push(assistant_message.clone());
        Ok(assistant_message.content)
    }

    async fn process_document(&mut self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Create a prompt that includes the document content
        let prompt = format!("Please analyze this document:\n\n{}", content);
        
        self.send_message(prompt).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        println!("OPENAI API key:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

    // Initialize chat app with OpenAI by default
    let mut chat_app = ChatApp::new(
        AiProvider::OpenAI,
        api_key,
        "gpt-3.5-turbo".to_string(),
    );

    println!("Welcome to the AI Chat Terminal!");
    println!("Commands:");
    println!("  /doc <filepath> - Submit a document for analysis");
    println!("  /switch - Switch between OpenAI and Ollama");
    println!("  /quit - Exit the application");
    println!("Enter your message:");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "/quit" {
            break;
        } else if input.starts_with("/doc ") {
            let file_path = input.trim_start_matches("/doc ").trim();
            if Path::new(file_path).exists() {
                match chat_app.process_document(file_path).await {
                    Ok(response) => println!("AI: {}", response),
                    Err(e) => println!("Error processing document: {}", e),
                }
            } else {
                println!("File not found: {}", file_path);
            }
        } else if input == "/switch" {
            chat_app.provider = match chat_app.provider {
                AiProvider::OpenAI => {
                    println!("Switched to Ollama");
                    chat_app.model = "llama2".to_string(); // Default Ollama model
                    AiProvider::Ollama
                }
                AiProvider::Ollama => {
                    println!("Switched to OpenAI");
                    chat_app.model = "gpt-3.5-turbo".to_string();
                    AiProvider::OpenAI
                }
            };
        } else {
            match chat_app.send_message(input.to_string()).await {
                Ok(response) => println!("AI: {}", response),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    println!("Bye.");
    Ok(())
}
