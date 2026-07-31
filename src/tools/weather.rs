use std::error::Error;
use super::Tool;

pub struct WeatherTool;

impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "get_weather"
    }

    fn description(&self) -> &str {
        "Returns the current weather for a location."
    }

    fn args_schema(&self) -> Vec<&str> {
        vec!["location"]
    }

    fn execute(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
        if args.is_empty() {
            return Err("Missing argument: location".into());
        }
        
        let loc = args[0].to_lowercase();
        let weather = match loc.as_str() {
            "london" => "15°C, Rainy",
            "tokyo" => "22°C, Sunny",
            "new york" => "10°C, Cloudy",
            _ => "Weather not found."
        };
        
        Ok(weather.to_string())
    }
}
