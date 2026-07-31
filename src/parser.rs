use regex::Regex;

#[derive(Debug)]
pub struct ParserResult {
    pub is_final: bool,
    pub text: String,
    pub tool_name: Option<String>,
    pub tool_args: Vec<String>,
}

pub struct ForgivingParser;

impl ForgivingParser {
    pub fn parse(text: &str) -> ParserResult {
        // Look for answer tag first
        if let Ok(re) = Regex::new(r"(?is)<answer>\s*(.*?)\s*</answer>") {
            if let Some(caps) = re.captures(text) {
                return ParserResult {
                    is_final: true,
                    text: caps[1].to_string(),
                    tool_name: None,
                    tool_args: vec![],
                };
            }
        }

        // Look for call tag
        if let Ok(call_re) = Regex::new(r"(?is)<call>\s*(.*?)\s*</call>") {
            if let Some(call_caps) = call_re.captures(text) {
                let tool_name = call_caps[1].trim().to_string();
                let mut tool_args = Vec::new();
                
                if let Ok(arg_re) = Regex::new(r"(?is)<arg>\s*(.*?)\s*</arg>") {
                    for cap in arg_re.captures_iter(text) {
                        tool_args.push(cap[1].trim().to_string());
                    }
                }

                return ParserResult {
                    is_final: false,
                    text: text.to_string(),
                    tool_name: Some(tool_name),
                    tool_args,
                };
            }
        }

        // Neither found. Return the text as a final answer to prevent infinite loops.
        ParserResult {
            is_final: true,
            text: text.trim().to_string(),
            tool_name: None,
            tool_args: vec![],
        }
    }
}
