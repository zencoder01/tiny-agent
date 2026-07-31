# Tiny Agent Harness

A highly optimized, zero-overhead Rust harness for running agentic loops on sub-500M parameter Language Models.

Small language models (e.g., Qwen1.5-0.5B, Smollm, MobileLLM) are exceptionally fast but struggle with complex JSON schemas, deep reasoning loops, and long context windows. This project provides a specialized harness that does the heavy lifting outside of the model, allowing tiny models to use tools reliably.

## Key Features

- **Zero-Overhead Execution**: Written in Rust (`tokio`, `reqwest`), the harness adds virtually no latency on top of the model inference.
- **Forgiving XML Parser**: Tiny models often fail to close JSON brackets or format structures correctly. This harness uses a regex-based XML tag extractor (`<call>`, `<arg>`) that safely recovers intentions even when the model hallucinates formatting.
- **Trait-Based Tool Registry**: Easily extend the agent's capabilities by implementing the `Tool` trait.
- **Single Binary**: Compiles down to a small, standalone executable.

---

## Architecture

The harness manages the ReAct (Reasoning and Acting) loop by injecting a highly compressed, aggressive system prompt and managing the tool execution state.

```mermaid
flowchart TD
    Client[User Prompt] --> Harness(MicroAgent Harness)
    
    subgraph Agent Loop
        Harness --> |Conversation History| LLM[Local LLM Endpoint]
        LLM --> |Raw Response| Parser{Forgiving XML Parser}
        
        Parser -->|<call> tag detected| Execution[Execute Tool]
        Execution -->|<observation> result| Harness
        
        Parser -->|<answer> tag detected| Return[Return Final Answer]
    end
    
    Execution -.-> Tools[(Tool Registry)]
```

---

## The XML Protocol

Instead of forcing the model to generate strict JSON strings, the harness prompts the model to use simple pseudo-XML tags.

**Example Model Output:**
```xml
<thought>I need to find the current weather in Tokyo.</thought>
<call>get_weather</call>
<arg>tokyo</arg>
```

**Harness Injection:**
```xml
<observation>22°C, Sunny</observation>
Now provide your <answer> or make another <call>.
```

**Final Model Output:**
```xml
<answer>The current weather in Tokyo is 22°C and Sunny.</answer>
```

---

## Usage

### 1. Requirements

- [Rust Toolchain](https://rustup.rs/) (1.70+)
- A local LLM runner (e.g., [Ollama](https://ollama.com/), vLLM) exposing an OpenAI-compatible `/v1/chat/completions` endpoint.

### 2. Configuration

By default, the harness connects to `http://localhost:11434/v1` (the default Ollama port). Ensure you have a small model pulled and running. For example:

```bash
ollama run qwen2.5:0.5b
```

### 3. Build and Run

```bash
cargo build --release
cargo run --release
```

---

## Extending the Harness (Adding Tools)

To add a new tool, implement the `Tool` trait and register it with the `MicroAgent`.
(Note: The included `WeatherTool` currently returns mocked data for demonstration purposes).

```rust
use tiny_agent::tools::Tool;
use std::error::Error;

pub struct MyCustomTool;

impl Tool for MyCustomTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "Does a custom action."
    }

    fn args_schema(&self) -> Vec<&str> {
        vec!["argument_1"]
    }

    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        // Note: For demonstration purposes, you would return mocked data here
        // or make an HTTP request to a real weather API.
        Ok(format!("Executed with {}", args[0]))
    }
}
```

Register it in `main.rs`:
```rust
agent.register_tool(Box::new(MyCustomTool));
```

---

## License

MIT License
