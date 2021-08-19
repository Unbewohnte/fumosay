use std::fs;
use std::path::Path;
use clap::{Arg, App}; 

/// Indicator of where the message should be in fumofile
const MESSAGE_INDICATOR: &str = "!message";
/// Fumofile name of the default fumo
const FUMO_DEFAULT: &str = "fumo.fumo"; 
/// Default directory where fumofiles are placed
const FUMOFILES_DIR: &str = "/usr/share/fumosay/fumofiles/";

/// Returns a resulting string with `MESSAGE_INDICATOR` replaced with given
/// `message`. If `MESSAGE_INDICATOR` is not present in fumofile - the
/// `message` will be added on the new line at the end of the fumofile. 
fn process_message(fumofile_contents:  &mut String, message: &str) -> String {
    if fumofile_contents.contains(MESSAGE_INDICATOR) {
        return fumofile_contents.replace(MESSAGE_INDICATOR,message);
    }
    return format!("{}\n{}", fumofile_contents, message);
}

fn main() {
    // get command line arguments
    let matches = App::new("fumosay")
        .version("0.3.0")
        .author("Unbewohnte | Nikolay Kasyanov <https://github.com/Unbewohnte>")
        .about("cowsay, but with soft friends")
        .arg(
            Arg::with_name("fumo")
            .short("f")
            .long("fumo")
            .value_name("fumofile")
            .help("choose another fumofumo to print")
            .takes_value(true)
            .required(false)
            .default_value(FUMO_DEFAULT)
        )
        .arg(
            Arg::with_name("fumofiles_directory")
            .short("d")
            .long("fumofiles_directory")
            .value_name("fumofiles_directory_path")
            .help("look for fumofiles in given directory instead")
            .takes_value(true)
            .required(false)
            .default_value(FUMOFILES_DIR)
        )
        .arg(
            Arg::with_name("message")
            .short("m")
            .long("message")
            .help("set a message to print")
            .takes_value(true)
            .required(true)
            .index(1)
            .multiple(true)
        ).get_matches();

    // process fumofiles directory
    
    // directory with all fumofiles
    let new_fumofiles_dir = matches.value_of("fumofiles_directory").unwrap();
    // name of the fumofile to work with
    let fumofile_name = matches.value_of("fumo").unwrap();
    // the whole path to the selected fumo
    let fumofile_path = Path::new(&new_fumofiles_dir).join(fumofile_name);
    
    // read fumofile
    let mut fumofile_contents: String = fs::read_to_string(fumofile_path).expect("Could not find a fumofile!");
    
    // parse the file and get the resulting string
    let message: String = process_message(&mut fumofile_contents, matches.value_of("message").unwrap());
    
    println!("{}", message);
}


