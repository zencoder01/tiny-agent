use regex::Regex;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_formed_answer() {
        let input = "<answer>The answer is 42.</answer>";
        let res = ForgivingParser::parse(input);
        assert_eq!(res.is_final, true);
        assert_eq!(res.text, "The answer is 42.");
        assert_eq!(res.tool_name, None);
    }

    #[test]
    fn test_well_formed_call_multiple_args() {
        let input = "<call>calculate</call><arg>2 + 2</arg><arg>extra</arg>";
        let res = ForgivingParser::parse(input);
        assert_eq!(res.is_final, false);
        assert_eq!(res.tool_name, Some("calculate".to_string()));
        assert_eq!(res.tool_args, vec!["2 + 2", "extra"]);
    }

    #[test]
    fn test_malformed_unclosed_tags() {
        // Even if tags are slightly malformed, we try to parse.
        // Actually, our regex currently requires the closing tag.
        // If it's missing, it should fall back to plain text.
        let input = "<call>calculate</call><arg>2 + 2";
        let res = ForgivingParser::parse(input);
        assert_eq!(res.is_final, false);
        assert_eq!(res.tool_name, Some("calculate".to_string()));
        // Since <arg> isn't closed, it misses the argument
        assert_eq!(res.tool_args.len(), 0);
    }

    #[test]
    fn test_plain_text_no_tags() {
        let input = "I am just talking without tags.";
        let res = ForgivingParser::parse(input);
        assert_eq!(res.is_final, true);
        assert_eq!(res.text, "I am just talking without tags.");
        assert_eq!(res.tool_name, None);
    }
}
