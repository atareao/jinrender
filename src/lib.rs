pub use minijinja::{context, Environment};
pub use std::{collections::HashMap, env::vars, fs::File, io::Read, io::Write, path::PathBuf};

/// Read template file contents
pub fn read_template_file(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Get environment variables as HashMap
pub fn get_environment_variables() -> HashMap<String, String> {
    let mut env = HashMap::new();
    for (key, value) in vars() {
        env.insert(key, value);
    }
    env
}

/// Render a Jinja template with environment variables
pub fn render_template(
    template_name: &str,
    template_content: &str,
    env_vars: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut environment = Environment::new();
    environment.add_template(template_name, template_content)?;

    let template = environment.get_template(template_name)?;
    let ctx = context!(env => env_vars);

    Ok(template.render(&ctx)?)
}

/// Write rendered content to output file
pub fn write_output_file(
    output_path: &PathBuf,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_file = File::create(output_path)?;
    output_file.write_all(content.as_bytes())?;
    Ok(())
}

/// Process a template file and write the rendered output
pub fn process_template(
    jinja_file: &PathBuf,
    output_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing template: {}", jinja_file.display());
    println!("Output file: {}", output_file.display());

    let contents = read_template_file(jinja_file)?;
    let env_vars = get_environment_variables();
    let rendered = render_template(jinja_file.to_str().unwrap(), &contents, &env_vars)?;
    write_output_file(output_file, &rendered)?;

    Ok(())
}
