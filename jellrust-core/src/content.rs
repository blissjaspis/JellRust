// Re-export types from jellrust-types
pub use jellrust_types::{FrontMatter, Page, Post, Site};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_parse_post_date() {
        let post = Post::new(PathBuf::from("_posts/2024-01-15-test-post.md"));
        let date = post.parse_date_from_filename().unwrap();
        assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-01-15");
    }
}

