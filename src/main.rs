pub mod parser;
pub mod tools;
pub mod harness;

use harness::MicroAgent;
use tools::math::CalculatorTool;
use tools::weather::WeatherTool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Tiny Agent Harness 🤖");
    println!("Ensure Ollama is running at localhost:11434 with 'qwen2.5:0.5b' pulled.");
    println!("{:-<50}", "");

    // Using Ollama as the local backend.
    let mut agent = MicroAgent::new("qwen2.5:0.5b", "http://localhost:11434/v1");

    // Register tools
    agent.register_tool(Box::new(CalculatorTool));
    agent.register_tool(Box::new(WeatherTool));

    let question = "What is the weather in Tokyo, and what is 15 * 8?";
    println!("\nUser: {}", question);

    let final_answer = agent.run(question).await?;

    println!("{:-<50}", "");
    println!("\nFinal Answer:\n{}", final_answer);

    Ok(())
}
