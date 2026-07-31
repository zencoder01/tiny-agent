pub mod parser;
pub mod tools;
pub mod harness;
use harness::MicroAgent;
use tools::math::CalculatorTool;
use tools::weather::WeatherTool;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "qwen2.5:0.5b")]
    model: String,

    #[arg(short, long, default_value = "http://localhost:11434/v1")]
    base_url: String,

    #[arg(short, long, default_value = "not-needed")]
    api_key: String,

    #[arg(short = 's', long, default_value_t = 5)]
    max_steps: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("🤖 Tiny Agent Harness 🤖");
    println!("Ensure Ollama is running at localhost:11434 with 'qwen2.5:0.5b' pulled.");
    println!("{:-<50}", "");

    // Using configuration from arguments.
    let mut agent = MicroAgent::new(args.model, args.base_url, args.api_key, args.max_steps);

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
