use std::fs;
use std::error::Error;
use webbrowser;
use colored::*;

pub fn create_web_dev_folder(filename: &str) -> Result<(), Box<dyn Error>> {
    //create a index.html file
    fs::File::create(format!("{}/index.html", filename))?;
    //create a public directory
    match fs::create_dir(format!("{}/public", filename)) {
        Ok(_) => {
            fs::create_dir_all(format!("{}/public/js/main.js", filename))?;
            fs::create_dir_all(format!("{}/public/css/style.css", filename))?;
            fs::create_dir_all(format!("{}/public/assets/img", filename))?;
        }
        Err(err) => println!("{}", err)
    }
    Ok(())
}

pub fn open_stackoverflow(error: &str) -> Result<(),()> {
    if webbrowser::open(&format!("https://stackoverflow.com/search?q={}", error)[..]).is_ok() {
        Ok(())
    }
    else {
        Err(())
    }
}

pub fn open_google(input: &str) -> Result<(),()> {
	if webbrowser::open(&format!("https://www.google.com/search?q={}", input)[..]).is_ok() {
        Ok(())
    }
    else {
        Err(())
    }
}