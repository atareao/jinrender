use std::{
    env::vars,
    path::PathBuf,
    fs::File,
    io::Read,
    io::Write,
    collections::HashMap,
};
use minijinja::{Environment, context};
use clap::{arg, Command, value_parser};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("jinrender")
        .version("0.1.0")
        .about("Render jinja templates")
        .arg(
            arg!(-j --jinja <FILE> "Set the jinja file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-o --output <FILE> "Set the output file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
    .get_matches();
    if let Some(jinja_file) = matches.get_one::<PathBuf>("jinja") {
        if let Some(output_file) = matches.get_one::<PathBuf>("output") {
            println!("Value for jinja file: {}", jinja_file.display());
            println!("Value for output file: {}", output_file.display());
            let mut file = File::open(jinja_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let rendered = reder_template(jinja_file.to_str().unwrap(), &contents)?;
            let mut output_file = File::create(output_file)?;
            output_file.write_all(rendered.as_bytes())?;
        }
    }
    Ok(())
}

fn reder_template(jinja_file: &str, jinja_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut env = HashMap::new();
    for (key, value) in vars() {
        env.insert(key, value);
    }
    let mut environment = Environment::new();
    environment.add_template(jinja_file, jinja_content)?;

    let template = environment.get_template(jinja_file)?;
    let ctx = context!(env);

    Ok(template.render(&ctx)?)
}
