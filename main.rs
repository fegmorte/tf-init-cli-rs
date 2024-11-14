use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Welcome to the Terraform Init CLI!");

    // Get AWS region
    let aws_region = prompt("Enter AWS region (e.g., us-west-2): ")?;

    // Get backend type
    let backend_type = prompt("Enter backend type (e.g., cloud, local): ")?;

    // Get template directory
    let template_dir = prompt("Enter the template directory path: ")?;

    // Get output directory
    let output_dir = prompt("Enter the output directory path: ")?;

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Copy template files to output directory
    copy_dir_all(&template_dir, &output_dir)?;

    // Modify the provider configuration file
    let provider_file = Path::new(&output_dir).join("provider.tf");
    let mut content = fs::read_to_string(&provider_file)?;
    content = content.replace("{{AWS_REGION}}", &aws_region);
    fs::write(&provider_file, content)?;

    // Copy the appropriate backend configuration file
    let backend_template = Path::new(&template_dir).join(format!("backend_{}.tf", backend_type));
    let backend_dest = Path::new(&output_dir).join("backend.tf");
    if backend_template.exists() {
        fs::copy(&backend_template, &backend_dest)?;
        
        // Remove the backend template files
        for entry in fs::read_dir(&output_dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            if file_name.to_str().map_or(false, |s| s.starts_with("backend_") && s.ends_with(".tf")) {
                fs::remove_file(entry.path())?;
            }
        }
    } else {
        println!("Warning: Backend template file '{}' not found.", backend_template.display());
    }

    println!("Terraform folder generated successfully in '{}'!", output_dir);
    Ok(())
}

fn prompt(message: &str) -> io::Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
