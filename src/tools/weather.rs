use std::error::Error;
use super::Tool;

/// A tool that returns the current weather for a given location.
/// Note: This currently returns hardcoded/mocked data for demonstration purposes.
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
        let loc = args.get(0).ok_or("Missing argument: location")?.to_lowercase();
        let weather = match loc.as_str() {
            "london" => "15°C, Rainy",
            "tokyo" => "22°C, Sunny",
            "new york" => "10°C, Cloudy",
            _ => "Weather not found."
        };
        
        Ok(weather.to_string())
    }
}
