use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let matches = App::new("Ms.Hudson")
        .version("1.0")
        .author("ProCode <bpro249@gmail.com>")
        .about("lets sherlock focus on whats important, meanwhile I'll make the hot water!")
        .arg(
            Arg::with_name("New_Project")
                .short("n")
                .long("newproject")
                .value_name("filename")
                .help("creates a project folder for you")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Webdev project")
                .short("w")
                .long("webdev")
                .help("creates html, css, js files within project")
                .takes_value(false),
        )
        .get_matches();
    let value = matches.value_of("New_Project").unwrap_or("newproject");
    match fs::create_dir(value) {
        Ok(_) => {
            if matches.is_present("Webdev project") {
                fs::File::create(format!("{}/index.html", value))?;
                fs::File::create(format!("{}/style.css", value))?;
                fs::File::create(format!("{}/index.js", value))?;
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    println!("{}", value);
    Ok(())
}
