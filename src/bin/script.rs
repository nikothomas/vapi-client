use std::fs;
use std::path::Path;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // Path to your models folder
    let models_dir = "./src/models";

    // Process all Rust files in the models directory
    process_directory(models_dir)?;

    println!("Successfully updated all model files with OpenApi derive!");
    Ok(())
}

fn process_directory(dir_path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            process_directory(path.to_str().unwrap())?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            // Process only Rust files
            process_file(&path)?;
        }
    }

    Ok(())
}

fn process_file(file_path: &Path) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    // Patterns to check and modify
    let derive_pattern = Regex::new(r"#\[derive\(([^)]*)\)\]").unwrap();
    let import_section_pattern = Regex::new(r"(?s)(use [^;]+;(\n|[ \t]*\r\n|\r[ \t]*))+").unwrap();

    // Check if OpenApi is already in derives
    let content_has_openapi = derive_pattern.captures_iter(&content)
        .any(|cap| cap[1].contains("OpenApi"));

    // Check if utoipa is already imported
    let content_has_utoipa_import = content.contains("use utoipa::OpenApi;");

    // If both are already present, no need to modify
    if content_has_openapi && content_has_utoipa_import {
        println!("File already has OpenApi derive: {}", file_path.display());
        return Ok(());
    }

    // Modify the content
    let mut modified_content = content.clone();

    // Add utoipa import if not present
    if !content_has_utoipa_import {
        // Find the import section
        if let Some(import_match) = import_section_pattern.find(&content) {
            // Insert after the last import statement
            let insert_pos = import_match.end();
            modified_content.insert_str(insert_pos, "use utoipa::OpenApi;\n\n");
        } else {
            // If no imports found, add after file comments/docstrings
            // First try to find the end of the initial comment block
            let comment_end = content.find("*/");
            if let Some(pos) = comment_end {
                let insert_pos = pos + 2;
                let line_break = if content[insert_pos..].starts_with("\n") { "" } else { "\n" };
                modified_content.insert_str(insert_pos, &format!("{line_break}\nuse utoipa::OpenApi;\n"));
            } else {
                // If no comment block, add at the beginning of the file
                modified_content = format!("use utoipa::OpenApi;\n\n{}", modified_content);
            }
        }
    }

    // Add OpenApi to derives if not present
    if !content_has_openapi {
        modified_content = derive_pattern.replace_all(&modified_content, |caps: &regex::Captures| {
            let current_derives = &caps[1];
            if current_derives.contains("OpenApi") {
                return caps[0].to_string();
            }

            // Format properly with or without existing derives
            let new_derives = if current_derives.trim().is_empty() {
                "OpenApi".to_string()
            } else {
                format!("{}, OpenApi", current_derives)
            };

            format!("#[derive({})]", new_derives)
        }).to_string();
    }

    // Write the modified content back to the file
    fs::write(file_path, modified_content)?;
    println!("Updated file: {}", file_path.display());

    Ok(())
}