# AI Chat Terminal

A command-line interface application built in Rust that enables interaction with AI models (OpenAI and Ollama) for both chat conversations and document analysis.

## Features

- 💬 Interactive chat with AI models
- 📄 Document analysis capabilities
- 🔄 Support for multiple AI providers:
  - OpenAI (GPT-3.5-turbo)
  - Ollama (local models)
- 📝 Conversation history tracking
- 🔌 Easy switching between AI providers
- 🚀 Asynchronous processing

## Prerequisites

Before running the application, make sure you have:

- Rust and Cargo installed (https://rustup.rs/)
- OpenAI API key (if using OpenAI)
- Ollama installed and running locally (if using Ollama)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-ai-chat
cd rust-ai-chat
```

2. Add the required dependencies to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
```

3. Build the project:
```bash
cargo build --release
```

## Configuration

### OpenAI Setup
Set your OpenAI API key as an environment variable:
```bash
export OPENAI_API_KEY='your-api-key-here'
```

### Ollama Setup
1. Install Ollama from their official website
2. Start the Ollama service
3. No additional configuration needed - the application will connect to the default local endpoint

## Usage

1. Start the application:
```bash
cargo run --release
```

2. Available commands:
- Send a message: Simply type your message and press Enter
- Submit a document: `/doc <filepath>`
- Switch AI provider: `/switch`
- Exit the application: `/quit`

### Example Usage

```bash
> Hello, how can you help me today?
AI: I'm an AI assistant that can help you with various tasks including answering questions...

> /doc /path/to/document.txt
AI: [Analysis of the document content...]

> /switch
Switched to Ollama

> What's the current model?
AI: I'm running on the local Llama2 model...
```

## Architecture

The application is structured around the `ChatApp` struct which handles:
- Message history management
- API communication with AI providers
- Document processing
- Provider switching logic

Key components:
- `Message`: Represents a single message in the conversation
- `ChatRequest`: Structures requests to AI providers
- `ChatResponse`: Handles AI provider responses
- `AiProvider`: Enum for supported AI services

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenAI for their ChatGPT API
- Ollama for their local AI model hosting solution

## Roadmap

- [ ] Add support for more AI providers
- [ ] Implement conversation saving/loading
- [ ] Add streaming responses
- [ ] Support for different document formats
- [ ] Add configuration file support
- [ ] Implement markdown rendering
- [ ] Add unit tests

## Support

If you encounter any issues or have questions, please file an issue on the GitHub repository.
