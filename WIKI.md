# Tiny Agent Harness Wiki

Welcome to the comprehensive wiki for the Tiny Agent Harness project. This document provides in-depth information about the architecture, usage, customization, and best practices.

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Deep Dive](#architecture-deep-dive)
3. [Installation Guide](#installation-guide)
4. [Configuration Options](#configuration-options)
5. [XML Protocol Specification](#xml-protocol-specification)
6. [Tool Development Guide](#tool-development-guide)
7. [Built-in Tools](#built-in-tools)
8. [Running with Different LLMs](#running-with-different-llms)
9. [Performance Optimization](#performance-optimization)
10. [Troubleshooting](#troubleshooting)
11. [Best Practices](#best-practices)
12. [Examples](#examples)
13. [API Reference](#api-reference)
14. [Contributing](#contributing)
15. [FAQ](#faq)

---

## Overview

### What is Tiny Agent Harness?

Tiny Agent Harness is a highly optimized, zero-overhead Rust harness designed specifically for running agentic loops on sub-500M parameter Language Models. It enables small language models to reliably use tools and perform complex reasoning tasks despite their inherent limitations.

### Why Small Models?

Small language models (SLMs) like Qwen1.5-0.5B, Smollm, and MobileLLM offer several advantages:
- **Speed**: 10-100x faster inference than large models
- **Cost**: Can run locally on consumer hardware
- **Privacy**: No data leaves your machine
- **Efficiency**: Lower memory and power consumption

However, they struggle with:
- Complex JSON schemas
- Deep reasoning loops
- Long context windows
- Precise formatting requirements

Tiny Agent Harness solves these problems by doing the heavy lifting outside the model.

### Key Features

| Feature | Description |
|---------|-------------|
| **Zero-Overhead Execution** | Written in Rust with `tokio` and `reqwest`, adding minimal latency |
| **Forgiving XML Parser** | Regex-based tag extractor that handles malformed output gracefully |
| **Trait-Based Tool Registry** | Easy extension through the `Tool` trait implementation |
| **Single Binary** | Compiles to a standalone executable (~5-10MB) |
| **OpenAI-Compatible** | Works with any OpenAI-compatible API endpoint |
| **Async Runtime** | Built on Tokio for high-performance async operations |

---

## Architecture Deep Dive

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    User/Client Layer                        │
│                  (Prompts & Queries)                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   MicroAgent Harness                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Conversation Manager                      │  │
│  │  • Maintains chat history                             │  │
│  │  • Injects observations                                │  │
│  │  • Manages step counting                               │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Forgiving XML Parser                      │  │
│  │  • Extracts <call>, <arg>, <thought>, <answer>        │  │
│  │  • Handles malformed tags                              │  │
│  │  • Regex-based recovery                                │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                 Tool Registry                          │  │
│  │  • Dynamic tool registration                           │  │
│  │  • Trait-based interface                               │  │
│  │  • Tool discovery                                      │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   LLM Endpoint                              │
│         (Local Ollama / Custom API Gateway)                 │
└─────────────────────────────────────────────────────────────┘
```

### The ReAct Loop

The harness implements the ReAct (Reasoning + Acting) pattern:

1. **Thought**: Model reasons about the current state
2. **Action**: Model decides to call a tool
3. **Observation**: Harness executes tool and returns result
4. **Repeat**: Cycle continues until answer is reached

### Module Structure

```
src/
├── main.rs          # Entry point and CLI handling
├── harness.rs       # Core agent loop and conversation management
├── parser.rs        # XML parsing and tag extraction
└── tools/
    ├── mod.rs       # Tool trait definition and registry
    └── weather.rs   # Example weather tool implementation
```

---

## Installation Guide

### Prerequisites

1. **Rust Toolchain** (version 1.70 or higher)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **LLM Runner** (one of the following):
   - [Ollama](https://ollama.com/) (recommended for local use)
   - vLLM
   - Any OpenAI-compatible API server

### Building from Source

```bash
# Clone the repository
git clone https://github.com/zencoder01/tiny-agent
cd tiny-agent-harness

# Build in release mode (optimized)
cargo build --release

# Binary location
./target/release/tiny-agent-harness
```

### System-Wide Installation

```bash
# Install to ~/.cargo/bin
cargo install --path .

# Now you can run from anywhere
tiny-agent-harness
```

### Docker Deployment (Optional)

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/tiny-agent-harness /usr/local/bin/
CMD ["tiny-agent-harness"]
```

---

## Configuration Options

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LLM_BASE_URL` | `http://localhost:11434/v1` | Base URL for LLM endpoint |
| `LLM_API_KEY` | `not-needed` | API key for authentication |
| `DEFAULT_MODEL` | `qwen2.5:0.5b` | Default model to use |
| `MAX_STEPS` | `5` | Maximum reasoning steps |

### Command Line Arguments

```bash
tiny-agent-harness [OPTIONS]

Options:
  -m, --model <MODEL>              Model name to use [default: qwen2.5:0.5b]
  -b, --base-url <URL>             Base URL of LLM endpoint [default: http://localhost:11434/v1]
  -k, --api-key <KEY>              API key for authentication [default: not-needed]
  -s, --max-steps <STEPS>          Maximum reasoning steps [default: 5]
      --api-gateway <GATEWAY>      Custom API gateway endpoint (overrides base-url)
  -h, --help                       Print help information
  -V, --version                    Print version information
```

### Configuration Examples

**Local Ollama Setup:**
```bash
tiny-agent-harness \
  --model qwen2.5:0.5b \
  --base-url http://localhost:11434/v1
```

**Production API Gateway:**
```bash
tiny-agent-harness \
  --model production-model-v1 \
  --api-gateway https://api.company.com/v1/chat/completions \
  --api-key $COMPANY_API_KEY \
  --max-steps 10
```

**Multiple Models Comparison:**
```bash
# Test with different models
tiny-agent-harness -m qwen2.5:0.5b
tiny-agent-harness -m smollm:135m
tiny-agent-harness -m mobilellm:350m
```

---

## XML Protocol Specification

### Tag Types

The harness recognizes four types of XML-like tags:

#### 1. `<thought>` - Reasoning Step
Indicates the model's internal reasoning (not shown to user).

```xml
<thought>I need to find the weather in Tokyo first, then calculate what to wear.</thought>
```

#### 2. `<call>` - Tool Invocation
Specifies which tool to execute.

```xml
<call>get_weather</call>
```

#### 3. `<arg>` - Tool Arguments
Provides arguments to the tool (can appear multiple times).

```xml
<arg>tokyo</arg>
<arg>japan</arg>
```

#### 4. `<answer>` - Final Response
Returns the final answer to the user.

```xml
<answer>The weather in Tokyo is sunny with 22°C.</answer>
```

### Complete Example Flow

**User Query:**
```
What should I wear in Tokyo today?
```

**Model Response 1:**
```xml
<thought>I need to check the current weather in Tokyo to recommend appropriate clothing.</thought>
<call>get_weather</call>
<arg>tokyo</arg>
```

**Harness Injection:**
```xml
<observation>22°C, Sunny, Humidity: 60%</observation>
Now provide your <answer> or make another <call>.
```

**Model Response 2:**
```xml
<thought>With 22°C and sunny weather, light clothing would be appropriate.</thought>
<answer>I recommend wearing light clothing such as a t-shirt and light pants or shorts. 
It's sunny so you might want sunglasses and sunscreen. The temperature is comfortable at 22°C.</answer>
```

### Parsing Rules

The forgiving parser applies these rules:

1. **Tag Matching**: Uses regex patterns to find opening and closing tags
2. **Graceful Degradation**: Handles missing closing tags by inferring intent
3. **Nested Tag Handling**: Processes tags sequentially, not hierarchically
4. **Whitespace Tolerance**: Ignores extra whitespace and newlines
5. **Case Insensitivity**: Treats `<CALL>` same as `<call>`

### Error Recovery

If the model produces malformed output:

```xml
<!-- Missing closing tag -->
<call>get_weather
<!-- Parser recovers by finding next tag or end of line -->

<!-- Malformed nesting -->
<call><arg>tokyo</call></arg>
<!-- Parser extracts both tags independently -->
```

---

## Tool Development Guide

### The Tool Trait

All tools must implement the `Tool` trait:

```rust
use std::error::Error;

pub trait Tool: Send + Sync {
    /// Returns the unique name of the tool
    fn name(&self) -> &str;
    
    /// Returns a description of what the tool does
    fn description(&self) -> &str;
    
    /// Returns the schema/expected arguments
    fn args_schema(&self) -> Vec<&str>;
    
    /// Executes the tool with provided arguments
    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>>;
}
```

### Creating a Custom Tool

**Example: Calculator Tool**

```rust
// src/tools/calculator.rs
use crate::tools::Tool;
use std::error::Error;

pub struct CalculatorTool;

impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Performs basic arithmetic operations. Use format: 'number operator number' (e.g., '5 + 3')"
    }

    fn args_schema(&self) -> Vec<&str> {
        vec!["expression"]
    }

    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        if args.is_empty() {
            return Err("No expression provided".into());
        }

        let expression = &args[0];
        
        // Simple parser for demonstration
        // In production, use a proper math expression parser
        let result = self.evaluate_expression(expression)?;
        
        Ok(format!("{}", result))
    }
}

impl CalculatorTool {
    fn evaluate_expression(&self, expr: &str) -> Result<f64, Box<dyn Error>> {
        // Implementation here
        Ok(0.0)
    }
}
```

### Registering Tools

In `main.rs`:

```rust
use crate::tools::{Tool, MicroAgent};
use crate::tools::calculator::CalculatorTool;
use crate::tools::weather::WeatherTool;

fn main() {
    let mut agent = MicroAgent::new(config);
    
    // Register built-in tools
    agent.register_tool(Box::new(WeatherTool));
    
    // Register custom tools
    agent.register_tool(Box::new(CalculatorTool));
    
    // Run the agent
    agent.run();
}
```

### Tool Best Practices

1. **Keep Tools Focused**: Each tool should do one thing well
2. **Clear Descriptions**: Help the model understand when to use each tool
3. **Error Handling**: Return informative error messages
4. **Argument Validation**: Validate inputs before processing
5. **Thread Safety**: Implement `Send + Sync` for async execution

---

## Built-in Tools

### WeatherTool

**Purpose**: Retrieves weather information for a location (mocked in demo).

**Usage**:
```xml
<call>get_weather</call>
<arg>tokyo</arg>
```

**Arguments**:
- `location` (string): City or location name

**Returns**: Weather conditions as a formatted string

**Implementation Notes**:
- Currently returns mocked data for demonstration
- Can be extended to call real weather APIs (OpenWeatherMap, WeatherAPI, etc.)

### Adding More Tools

The harness is designed for easy extension. Common tool ideas:

- **SearchTool**: Web search integration
- **DatabaseTool**: SQL query execution
- **FileTool**: File system operations
- **APITool**: Generic HTTP API calls
- **DateTimeTool**: Current time and date operations
- **TranslationTool**: Language translation

---

## Running with Different LLMs

### Supported Model Sizes

| Model | Size | Recommended For | Performance |
|-------|------|-----------------|-------------|
| Qwen2.5 | 0.5B | General tasks, good balance | ⭐⭐⭐⭐⭐ |
| SmolLM | 135M | Ultra-fast, simple tasks | ⭐⭐⭐⭐⭐⭐ |
| MobileLLM | 350M | Mobile deployment | ⭐⭐⭐⭐ |
| TinyLlama | 1.1B | Complex reasoning | ⭐⭐⭐ |

### Ollama Setup

```bash
# Pull models
ollama pull qwen2.5:0.5b
ollama pull smollm:135m
ollama pull mobilellm:350m

# Run with specific model
ollama run qwen2.5:0.5b
```

### Custom API Endpoints

For production deployments:

```bash
# Using a custom gateway
tiny-agent-harness \
  --api-gateway https://your-api.example.com/v1/chat/completions \
  --api-key $YOUR_API_KEY
```

### Model-Specific Tuning

Different models may require prompt adjustments:

```rust
// In harness.rs, customize system prompt per model
let system_prompt = match model_name {
    "smollm:135m" => SMOLLM_SYSTEM_PROMPT,  // Simpler instructions
    "qwen2.5:0.5b" => QWEN_SYSTEM_PROMPT,   // Standard instructions
    _ => DEFAULT_SYSTEM_PROMPT,
};
```

---

## Performance Optimization

### Benchmarking

Measure performance with different configurations:

```bash
# Time a query
time tiny-agent-harness -m qwen2.5:0.5b <<EOF
What is the weather in Tokyo?
EOF
```

### Optimization Strategies

1. **Model Selection**: Choose the smallest model that meets accuracy requirements
2. **Step Limiting**: Set appropriate `--max-steps` to prevent infinite loops
3. **Batch Processing**: Process multiple queries concurrently
4. **Caching**: Cache frequent tool results
5. **Connection Pooling**: Reuse HTTP connections for tool APIs

### Memory Usage

Typical memory footprint:
- Harness binary: ~5-10 MB
- Runtime overhead: <50 MB
- Model memory: Depends on model size (0.5B ≈ 1-2 GB)

### Latency Breakdown

```
Total Latency = Model Inference + Parsing + Tool Execution + Network

Typical values:
- Model Inference (0.5B): 100-500ms
- XML Parsing: <1ms
- Tool Execution: 10-100ms
- Network (local): <10ms
```

---

## Troubleshooting

### Common Issues

#### 1. Connection Refused

**Error**: `Connection refused (os error 111)`

**Solution**:
```bash
# Ensure Ollama is running
ollama serve

# Or check your base URL
tiny-agent-harness --base-url http://localhost:11434/v1
```

#### 2. Model Not Found

**Error**: `model 'xyz' not found`

**Solution**:
```bash
# Pull the required model
ollama pull qwen2.5:0.5b

# Or specify an available model
tiny-agent-harness --model available-model
```

#### 3. Infinite Loops

**Symptom**: Agent keeps calling tools without providing answer

**Solution**:
```bash
# Reduce max steps
tiny-agent-harness --max-steps 3

# Improve system prompt to encourage answering
```

#### 4. Malformed XML

**Symptom**: Parser fails to extract tags

**Solution**:
- Check model output logs
- Adjust system prompt for clearer tag usage
- Consider fine-tuning the model on XML format

#### 5. Tool Execution Errors

**Error**: `Tool execution failed: ...`

**Solution**:
- Verify tool arguments match schema
- Check tool implementation for bugs
- Add better error handling in tool code

### Debug Mode

Enable verbose logging:

```bash
RUST_LOG=debug tiny-agent-harness
```

### Log Analysis

Key log messages to watch for:
- `Parsed tags`: Shows extracted XML tags
- `Executing tool`: Tool invocation details
- `Step N/M`: Current step in reasoning loop
- `Final answer`: Completed response

---

## Best Practices

### Prompt Engineering

1. **Be Specific**: Clear questions get better answers
2. **Provide Context**: Include relevant background information
3. **Guide Format**: Explicitly request XML tags if needed
4. **Iterate**: Refine prompts based on model behavior

### Tool Design

1. **Atomic Operations**: Each tool should perform a single function
2. **Idempotency**: Tools should be safe to call multiple times
3. **Fast Execution**: Keep tool execution under 1 second when possible
4. **Clear Errors**: Provide actionable error messages

### Production Deployment

1. **Rate Limiting**: Implement request throttling
2. **Monitoring**: Track latency, errors, and tool usage
3. **Fallbacks**: Have backup plans for tool failures
4. **Security**: Validate all inputs and sanitize outputs
5. **Logging**: Maintain audit trails for debugging

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_parser() {
        let input = "<call>test</call><arg>value</arg>";
        let parsed = parse_xml(input);
        assert_eq!(parsed.call, Some("test"));
    }

    #[test]
    fn test_tool_execution() {
        let tool = WeatherTool;
        let result = tool.execute(&["tokyo".to_string()]);
        assert!(result.is_ok());
    }
}
```

---

## Examples

### Example 1: Simple Weather Query

**Input**:
```
What's the weather in Paris?
```

**Expected Flow**:
```
Model: <thought>Need to check weather in Paris</thought>
       <call>get_weather</call>
       <arg>paris</arg>

Harness: <observation>18°C, Partly Cloudy</observation>

Model: <answer>The weather in Paris is currently 18°C with partly cloudy skies.</answer>
```

### Example 2: Multi-Step Reasoning

**Input**:
```
Compare the weather in Tokyo and London, then suggest which city is better for outdoor activities.
```

**Expected Flow**:
```
Model: <thought>I need to check weather in both cities first</thought>
       <call>get_weather</call>
       <arg>tokyo</arg>

Harness: <observation>22°C, Sunny</observation>

Model: <thought>Now I need London's weather</thought>
       <call>get_weather</call>
       <arg>london</arg>

Harness: <observation>15°C, Rainy</observation>

Model: <thought>Tokyo is warmer and sunnier, better for outdoors</thought>
       <answer>Based on current conditions, Tokyo (22°C, Sunny) is better for outdoor 
       activities than London (15°C, Rainy). Tokyo offers pleasant warm weather with sunshine, 
       while London is cooler with rain.</answer>
```

### Example 3: Programmatic Usage

```rust
use tiny_agent::{MicroAgent, Config, tools::WeatherTool};

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .model("qwen2.5:0.5b")
        .base_url("http://localhost:11434/v1")
        .max_steps(5)
        .build();

    let mut agent = MicroAgent::new(config);
    agent.register_tool(Box::new(WeatherTool));

    let response = agent.query("What's the weather in New York?").await;
    println!("{}", response);
}
```

---

## API Reference

### MicroAgent

```rust
pub struct MicroAgent {
    config: Config,
    tools: HashMap<String, Box<dyn Tool>>,
    history: Vec<Message>,
}

impl MicroAgent {
    pub fn new(config: Config) -> Self;
    pub fn register_tool(&mut self, tool: Box<dyn Tool>);
    pub async fn query(&mut self, prompt: &str) -> Result<String>;
    pub async fn run(&mut self);
    pub fn get_history(&self) -> &[Message];
    pub fn clear_history(&mut self);
}
```

### Config

```rust
pub struct Config {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub max_steps: usize,
    pub api_gateway: Option<String>,
}

impl Config {
    pub fn builder() -> ConfigBuilder;
}
```

### Tool Trait

```rust
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn args_schema(&self) -> Vec<&str>;
    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>>;
}
```

### Parser Functions

```rust
pub fn extract_call(text: &str) -> Option<String>;
pub fn extract_args(text: &str) -> Vec<String>;
pub fn extract_thought(text: &str) -> Option<String>;
pub fn extract_answer(text: &str) -> Option<String>;
```

---

## Contributing

### How to Contribute

1. **Fork the Repository**
2. **Create a Feature Branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make Your Changes**
4. **Run Tests**
   ```bash
   cargo test
   ```
5. **Format Code**
   ```bash
   cargo fmt
   ```
6. **Submit a Pull Request**

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Document public APIs with rustdoc comments
- Write tests for new features

### Pull Request Guidelines

- Keep PRs focused on a single feature/fix
- Include tests for new functionality
- Update documentation as needed
- Describe changes clearly in PR description

---

## FAQ

### Q: What's the smallest model that works well?

**A**: SmolLM (135M) works for very simple tasks, but Qwen2.5 (0.5B) provides the best balance of speed and capability for most use cases.

### Q: Can I use this with cloud-based LLMs?

**A**: Yes! Point `--base-url` or `--api-gateway` to any OpenAI-compatible endpoint, including cloud providers.

### Q: How do I add authentication?

**A**: Use the `--api-key` flag or set the `LLM_API_KEY` environment variable.

### Q: What if the model never provides an answer?

**A**: The `--max-steps` parameter limits the loop. Increase it if needed, or adjust the system prompt to encourage answering.

### Q: Can tools have side effects?

**A**: Yes, but be cautious. Tools should ideally be idempotent and safe to retry.

### Q: Is this suitable for production use?

**A**: Yes, with proper monitoring, rate limiting, and error handling. The harness is designed for reliability.

### Q: How do I handle long-running tools?

**A**: Implement async tool execution with timeouts. The harness uses Tokio for async operations.

### Q: Can I use multiple tools in one response?

**A**: The current implementation processes one tool call at a time, but you can modify the parser to support batch operations.

---

## Changelog

### Version 1.0.0 (Current)
- Initial release
- XML-based protocol
- Tool registry system
- Async runtime with Tokio
- OpenAI-compatible API support

### Planned Features
- [ ] Batch tool execution
- [ ] Conversation persistence
- [ ] Advanced caching layer
- [ ] Web UI for monitoring
- [ ] Plugin system for tools
- [ ] Fine-tuning utilities

---

## License

MIT License - See LICENSE file for details.

---

## Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: This wiki and README.md

---

*Last updated: $(date)*
