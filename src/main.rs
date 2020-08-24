use clap::{App, Arg};
use std::fs;
mod web_dev;
use web_dev::{ create_web_dev_folder };

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
    let filename = matches.value_of("New_Project").unwrap_or("newproject");
    match fs::create_dir(filename) {
        Ok(_) => {
            if matches.is_present("Webdev project") {
                //making a general web dev file structure
                create_web_dev_folder(&filename);
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    println!("{}", filename);
    Ok(())
}
