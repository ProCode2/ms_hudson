use std::fs;
use std::error::Error;


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
