use std::error::Error;

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn args_schema(&self) -> Vec<&str>;
    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>>;

    fn to_prompt_string(&self) -> String {
        format!(
            "- name: {}\n  description: {}\n  args: {:?}",
            self.name(),
            self.description(),
            self.args_schema()
        )
    }
}

pub mod math;
pub mod weather;
