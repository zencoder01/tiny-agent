use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

use super::parser::ForgivingParser;
use super::tools::Tool;

pub struct MicroAgent {
    tools: HashMap<String, Box<dyn Tool>>,
    model: String,
    base_url: String,
    api_key: String,
    max_steps: usize,
    client: Client,
}

impl MicroAgent {
    pub fn new(model: &str, base_url: &str) -> Self {
        Self {
            tools: HashMap::new(),
            model: model.to_string(),
            base_url: base_url.to_string(),
            api_key: "not-needed".to_string(),
            max_steps: 5,
            client: Client::new(),
        }
    }

    pub fn register_tool(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    fn build_system_prompt(&self) -> String {
        let mut tools_str = String::new();
        for tool in self.tools.values() {
            tools_str.push_str(&tool.to_prompt_string());
            tools_str.push('\n');
        }

        format!(
            "You are a helpful AI assistant. You have access to the following tools:\n\n\
            {}\n\
            To use a tool, you MUST use the exact following XML format:\n\
            <thought>explain why you are using the tool</thought>\n\
            <call>tool_name</call>\n\
            <arg>first argument</arg>\n\
            <arg>second argument</arg>\n\n\
            If you have the final answer, you MUST use the following format:\n\
            <answer>your final answer to the user</answer>\n\n\
            Do NOT output conversational text without using these tags. Keep your thoughts extremely short.",
            tools_str
        )
    }

    pub async fn run(&self, user_prompt: &str) -> Result<String, Box<dyn Error>> {
        let mut messages = vec![
            json!({"role": "system", "content": self.build_system_prompt()}),
            json!({"role": "user", "content": user_prompt}),
        ];

        for step in 0..self.max_steps {
            println!("\n--- Step {} ---", step + 1);

            let payload = json!({
                "model": self.model,
                "messages": messages,
                "temperature": 0.0,
            });

            let resp = self.client.post(&format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&payload)
                .send()
                .await?;

            let resp_json: serde_json::Value = resp.json().await?;
            let output = resp_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            println!("LLM Output:\n{}\n", output);

            messages.push(json!({"role": "assistant", "content": output}));

            let parsed = ForgivingParser::parse(&output);

            if parsed.is_final {
                println!("✅ Final Answer Reached");
                return Ok(parsed.text);
            }

            if let Some(tool_name) = parsed.tool_name {
                let observation = match self.tools.get(&tool_name) {
                    Some(tool) => {
                        match tool.execute(&parsed.tool_args) {
                            Ok(res) => res,
                            Err(e) => format!("Error executing {}: {}", tool_name, e),
                        }
                    }
                    None => format!("Error: Tool '{}' does not exist.", tool_name),
                };

                println!("🛠️  Executed {}({:?}) -> {}", tool_name, parsed.tool_args, observation);

                let observation_msg = format!(
                    "<observation>{}</observation>\nNow provide your <answer> or make another <call>.",
                    observation
                );
                messages.push(json!({"role": "user", "content": observation_msg}));
            }
        }

        Ok("Error: Maximum steps reached without final answer.".to_string())
    }
}
