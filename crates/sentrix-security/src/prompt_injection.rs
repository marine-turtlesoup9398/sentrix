use regex::Regex;

pub struct PromptInjectionDefender;

impl PromptInjectionDefender {
    pub fn sanitize_untrusted_text(input: &str) -> String {
        // Neutralize common prompt injection directives embedded in repository source files / commit messages
        let injection_patterns = [
            r"(?i)\bignore\s+(all\s+)?(previous\s+)?instructions\b",
            r"(?i)\bdisregard\s+previous\s+directives\b",
            r"(?i)\bsystem\s+prompt\s+override\b",
            r"(?i)\bexecute\s+shell\s+command\b",
            r"(?i)\byou\s+are\s+now\s+in\s+dan\s+mode\b",
        ];

        let mut sanitized = input.to_string();
        for pattern in &injection_patterns {
            if let Ok(re) = Regex::new(pattern) {
                sanitized = re
                    .replace_all(&sanitized, "[NEUTRALIZED_PROMPT_INJECTION_TEXT]")
                    .to_string();
            }
        }

        sanitized
    }

    pub fn wrap_untrusted_context(label: &str, content: &str) -> String {
        let clean = Self::sanitize_untrusted_text(content);
        format!("\n--- BEGIN UNTRUSTED REPOSITORY DATA ({}) ---\n{}\n--- END UNTRUSTED REPOSITORY DATA ({}) ---\n", label, clean, label)
    }
}
