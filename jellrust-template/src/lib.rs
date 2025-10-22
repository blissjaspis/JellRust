use anyhow::{Context, Result};
use jellrust_types::{Config, Page, Post, Site};
use liquid::model::{Object, Value};
use liquid::ParserBuilder;
use std::fs;
use std::path::PathBuf;

pub struct TemplateEngine {
    source_dir: PathBuf,
    parser: liquid::Parser,
}

impl TemplateEngine {
    pub fn new(source_dir: PathBuf) -> Self {
        let parser = ParserBuilder::with_stdlib()
            .build()
            .unwrap();
        
        Self {
            source_dir,
            parser,
        }
    }
    
    /// Render a post with its layout
    pub fn render_post(
        &self,
        post: &Post,
        site: &Site,
        config: &Config,
    ) -> Result<String> {
        let mut globals = Object::new();
        
        // Add site variables
        globals.insert("site".into(), self.site_to_value(site, config));
        
        // Add page variables (post data)
        globals.insert("page".into(), self.post_to_value(post));
        
        // Add content
        globals.insert("content".into(), Value::scalar(post.html.clone()));
        
        // Get layout name
        let layout_name = post
            .front_matter
            .layout
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("default");
        
        // Render with layout
        self.render_with_layout(&post.html, layout_name, &globals)
    }
    
    /// Render Liquid templates in page content (before Markdown processing)
    pub fn render_page_content(
        &self,
        content: &str,
        page: &Page,
        site: &Site,
        config: &Config,
    ) -> Result<String> {
        let mut globals = Object::new();

        // Add site variables
        globals.insert("site".into(), self.site_to_value(site, config));

        // Add page variables
        globals.insert("page".into(), self.page_to_value(page));

        // Process Liquid templates in the content
        let template = self.parser.parse(content)
            .with_context(|| format!("Failed to parse Liquid templates in page content"))?;

        template.render(&globals)
            .with_context(|| format!("Failed to render Liquid templates in page content"))
    }

    /// Render a page with its layout
    pub fn render_page(
        &self,
        page: &Page,
        site: &Site,
        config: &Config,
    ) -> Result<String> {
        let mut globals = Object::new();
        
        // Add site variables
        globals.insert("site".into(), self.site_to_value(site, config));
        
        // Add page variables
        globals.insert("page".into(), self.page_to_value(page));
        
        // Add content
        globals.insert("content".into(), Value::scalar(page.html.clone()));
        
        // Get layout name
        let layout_name = page
            .front_matter
            .layout
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("default");
        
        // Render with layout
        self.render_with_layout(&page.html, layout_name, &globals)
    }
    
    /// Render content with a layout
    fn render_with_layout(
        &self,
        content: &str,
        layout_name: &str,
        globals: &Object,
    ) -> Result<String> {
        let layout_path = self
            .source_dir
            .join("_layouts")
            .join(format!("{}.html", layout_name));
        
        if !layout_path.exists() {
            // No layout, return content as-is
            tracing::warn!("Layout not found: {}", layout_name);
            return Ok(content.to_string());
        }
        
        let layout_content = fs::read_to_string(&layout_path)
            .with_context(|| format!("Failed to read layout: {}", layout_path.display()))?;

        // Extract parent layout from front matter
        let parent_layout = self.extract_parent_layout(&layout_content);

        // Extract template content (strip front matter)
        let template_content = self.extract_template_content(&layout_content);

        // Parse and render the layout
        let template = self.parser.parse(template_content)
            .with_context(|| format!("Failed to parse layout: {}", layout_name))?;

        let output = template.render(globals)
            .with_context(|| format!("Failed to render layout: {}", layout_name))?;

        // Check if this layout has a parent layout
        if let Some(parent_layout) = parent_layout {
            let mut new_globals = globals.clone();
            new_globals.insert("content".into(), Value::scalar(output.clone()));
            return self.render_with_layout(&output, &parent_layout, &new_globals);
        }
        
        Ok(output)
    }
    
    /// Extract parent layout from front matter in layout file
    fn extract_parent_layout(&self, layout_content: &str) -> Option<String> {
        if !layout_content.trim().starts_with("---") {
            return None;
        }

        let rest = &layout_content.trim()[3..];
        if let Some(end_pos) = rest.find("\n---") {
            let yaml_content = &rest[..end_pos];
            if let Ok(data) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
                if let Some(layout) = data.get("layout") {
                    return layout.as_str().map(|s| s.to_string());
                }
            }
        }

        None
    }

    /// Extract template content from layout by stripping front matter
    fn extract_template_content<'a>(&self, layout_content: &'a str) -> &'a str {
        let trimmed = layout_content.trim();

        // Check if content starts with ---
        if !trimmed.starts_with("---") {
            return layout_content;
        }

        // Find the ending ---
        let rest = &trimmed[3..];
        if let Some(end_pos) = rest.find("\n---") {
            // Return everything after the front matter
            &rest[end_pos + 4..].trim_start()
        } else {
            // No closing --- found, return original content
            layout_content
        }
    }
    
    /// Convert Site to Liquid Value
    fn site_to_value(&self, site: &Site, config: &Config) -> Value {
        let mut obj = Object::new();
        
        // Add config values
        obj.insert("title".into(), Value::scalar(config.title.clone()));
        obj.insert("description".into(), Value::scalar(config.description.clone()));
        obj.insert("url".into(), Value::scalar(config.url.clone()));
        obj.insert("baseurl".into(), Value::scalar(config.baseurl.clone()));
        
        // Add posts
        let posts: Vec<Value> = site
            .posts
            .iter()
            .map(|p| self.post_to_value(p))
            .collect();
        obj.insert("posts".into(), Value::Array(posts));
        
        // Add pages
        let pages: Vec<Value> = site
            .pages
            .iter()
            .map(|p| self.page_to_value(p))
            .collect();
        obj.insert("pages".into(), Value::Array(pages));
        
        Value::Object(obj)
    }
    
    /// Convert Post to Liquid Value
    fn post_to_value(&self, post: &Post) -> Value {
        let mut obj = Object::new();
        
        obj.insert("url".into(), Value::scalar(post.url.clone()));
        obj.insert("date".into(), Value::scalar(post.date.to_rfc3339()));
        obj.insert("excerpt".into(), Value::scalar(post.excerpt.clone()));
        
        if let Some(title) = &post.front_matter.title {
            obj.insert("title".into(), Value::scalar(title.clone()));
        }
        
        if let Some(author) = &post.front_matter.author {
            obj.insert("author".into(), Value::scalar(author.clone()));
        }
        
        // Add categories
        let categories: Vec<Value> = post
            .front_matter
            .categories
            .iter()
            .map(|c| Value::scalar(c.clone()))
            .collect();
        obj.insert("categories".into(), Value::Array(categories));
        
        // Add tags
        let tags: Vec<Value> = post
            .front_matter
            .tags
            .iter()
            .map(|t| Value::scalar(t.clone()))
            .collect();
        obj.insert("tags".into(), Value::Array(tags));
        
        Value::Object(obj)
    }
    
    /// Convert Page to Liquid Value
    fn page_to_value(&self, page: &Page) -> Value {
        let mut obj = Object::new();
        
        obj.insert("url".into(), Value::scalar(page.url.clone()));
        
        if let Some(title) = &page.front_matter.title {
            obj.insert("title".into(), Value::scalar(title.clone()));
        }
        
        Value::Object(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_engine_creation() {
        let engine = TemplateEngine::new(PathBuf::from("."));
        assert!(engine.source_dir.ends_with("."));
    }
}

