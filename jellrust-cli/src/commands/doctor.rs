use anyhow::Result;
use std::path::PathBuf;

pub fn execute(source: PathBuf) -> Result<()> {
    println!("🔍 Running JellRust Doctor...\n");
    
    let mut issues = 0;
    let mut warnings = 0;
    
    // Check if _config.yml exists
    if !source.join("_config.yml").exists() {
        println!("❌ Missing _config.yml");
        issues += 1;
    } else {
        println!("✅ Found _config.yml");
    }
    
    // Check for _layouts directory
    if !source.join("_layouts").exists() {
        println!("⚠️  Missing _layouts directory");
        warnings += 1;
    } else {
        println!("✅ Found _layouts directory");
        
        // Check for default layout
        if !source.join("_layouts/default.html").exists() {
            println!("⚠️  No default.html layout found");
            warnings += 1;
        }
    }
    
    // Check for _posts directory
    if !source.join("_posts").exists() {
        println!("⚠️  Missing _posts directory");
        warnings += 1;
    } else {
        println!("✅ Found _posts directory");
    }
    
    // Check for index file
    let has_index = source.join("index.md").exists()
        || source.join("index.html").exists()
        || source.join("index.markdown").exists();
    
    if !has_index {
        println!("❌ No index file found (index.md, index.html, etc.)");
        issues += 1;
    } else {
        println!("✅ Found index file");
    }
    
    // Check for assets
    if source.join("assets").exists() {
        println!("✅ Found assets directory");
    } else {
        println!("⚠️  No assets directory found");
        warnings += 1;
    }
    
    // Summary
    println!("\n─────────────────────────");
    if issues == 0 && warnings == 0 {
        println!("✅ Your site looks good!");
    } else {
        if issues > 0 {
            println!("❌ Found {} critical issue(s)", issues);
        }
        if warnings > 0 {
            println!("⚠️  Found {} warning(s)", warnings);
        }
    }
    
    Ok(())
}

