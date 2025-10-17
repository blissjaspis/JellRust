use anyhow::{Context, Result};
use jellrust_core::content::FrontMatter;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use once_cell::sync::Lazy;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

pub struct MarkdownProcessor {
    options: Options,
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        Self { options }
    }
    
    /// Parse front matter and content from a markdown file
    pub fn parse_front_matter(&self, content: &str) -> Result<(FrontMatter, &str)> {
        let trimmed = content.trim();
        
        // Check if content starts with ---
        if !trimmed.starts_with("---") {
            return Ok((FrontMatter::default(), content));
        }
        
        // Find the ending ---
        let rest = &trimmed[3..];
        if let Some(end_pos) = rest.find("\n---") {
            let yaml_content = &rest[..end_pos];
            let body = &rest[end_pos + 4..].trim_start();
            
            // Parse YAML front matter
            let front_matter: FrontMatter = serde_yaml::from_str(yaml_content)
                .context("Failed to parse YAML front matter")?;
            
            Ok((front_matter, body))
        } else {
            // No closing ---, treat entire content as body
            Ok((FrontMatter::default(), content))
        }
    }
    
    /// Render Markdown to HTML
    pub fn render(&self, markdown: &str) -> Result<String> {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::new();
        
        // Process events for syntax highlighting
        let events = self.add_syntax_highlighting(parser);
        
        html::push_html(&mut html_output, events.into_iter());
        
        Ok(html_output)
    }
    
    /// Add syntax highlighting to code blocks
    fn add_syntax_highlighting<'a>(
        &self,
        parser: Parser<'a>,
    ) -> Vec<Event<'a>> {
        let mut events = Vec::new();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();
        
        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_block_lang = lang.to_string();
                    code_block_content.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    if in_code_block {
                        in_code_block = false;
                        
                        // Highlight the code
                        if let Some(highlighted) = self.highlight_code(&code_block_content, &code_block_lang) {
                            events.push(Event::Html(highlighted.into()));
                        } else {
                            // Fallback to plain code block
                            events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(code_block_lang.as_str().into()))));
                            events.push(Event::Text(code_block_content.clone().into()));
                            events.push(Event::End(TagEnd::CodeBlock));
                        }
                    } else {
                        events.push(event);
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        events.push(Event::Text(text));
                    }
                }
                _ => {
                    if !in_code_block {
                        events.push(event);
                    }
                }
            }
        }
        
        events
    }
    
    /// Highlight code using syntect
    fn highlight_code(&self, code: &str, lang: &str) -> Option<String> {
        let syntax = SYNTAX_SET
            .find_syntax_by_token(lang)
            .or_else(|| SYNTAX_SET.find_syntax_plain_text())?;
        
        let theme = &THEME_SET.themes["base16-ocean.dark"];
        
        highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme).ok()
    }
}

impl Default for MarkdownProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_front_matter() {
        let content = r#"---
title: Test Post
date: 2024-01-01
---

# Hello World

This is content."#;
        
        let processor = MarkdownProcessor::new();
        let (front_matter, body) = processor.parse_front_matter(content).unwrap();
        
        assert_eq!(front_matter.title, Some("Test Post".to_string()));
        assert!(body.contains("# Hello World"));
    }
    
    #[test]
    fn test_render_markdown() {
        let processor = MarkdownProcessor::new();
        let html = processor.render("# Hello\n\nThis is **bold**.").unwrap();
        
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }
    
    #[test]
    fn test_no_front_matter() {
        let content = "# Just content\n\nNo front matter here.";
        
        let processor = MarkdownProcessor::new();
        let (front_matter, body) = processor.parse_front_matter(content).unwrap();
        
        assert_eq!(front_matter.title, None);
        assert_eq!(body, content);
    }
}

