use clap::{arg, value_parser, Command};
use jinrender::process_template;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("jinrender")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Render jinja templates")
        .arg(
            arg!(-j --jinja <FILE> "Set the jinja file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <FILE> "Set the output file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(jinja_file) = matches.get_one::<PathBuf>("jinja") {
        if let Some(output_file) = matches.get_one::<PathBuf>("output") {
            process_template(jinja_file, output_file)?;
        }
    }
    Ok(())
}
