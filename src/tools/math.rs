use std::error::Error;
use super::Tool;

pub struct CalculatorTool;

impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculate"
    }

    fn description(&self) -> &str {
        "Evaluates a mathematical expression (e.g., '2 + 2'). Only supports basic math (+, -, *, /)."
    }

    fn args_schema(&self) -> Vec<&str> {
        vec!["expression"]
    }

    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        let expr = args.get(0).ok_or("Missing argument: expression")?;
        
        // In a real agent, we'd use a safe evaluator crate like `meval` or `fasteval`.
        // For demonstration, we'll just parse very basic `a op b`
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() >= 3 {
            let a: f64 = tokens[0].parse()?;
            let op = tokens[1];
            let b: f64 = tokens[2].parse()?;
            
            let res = match op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => return Err("Unsupported operator".into())
            };
            return Ok(res.to_string());
        }
        
        Ok(format!("Could not parse simple expression: {}", expr))
    }
}
