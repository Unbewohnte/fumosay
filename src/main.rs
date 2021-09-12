use std::fs;
use std::path::Path;
use clap::{Arg, App}; 

/// Indicator of where the message should be in fumofile
const MESSAGE_INDICATOR: &str = "!message";
/// Fumofile name of the default fumo
const FUMO_DEFAULT: &str = "cirno.fumo"; 
/// Default directory where fumofiles are placed
const DEFAULT_FUMOFILES_DIR: &str = "/usr/local/share/fumosay/fumofiles/";

/// Returns a resulting string with `MESSAGE_INDICATOR` replaced with given
/// `message`. If `MESSAGE_INDICATOR` is not present in fumofile - the
/// `message` will be added on the new line at the end of the fumofile. 
fn process_message(fumofile_contents:  &mut String, message: &str) -> String {
    if fumofile_contents.contains(MESSAGE_INDICATOR) {
        return fumofile_contents.replace(MESSAGE_INDICATOR, message);
    }
    return format!("{}\n{}", fumofile_contents, message);
}

fn main() {
    // get command line arguments
    let matches = App::new("fumosay")
        .version("0.4.2")
        .author("Unbewohnte | Nikolay Kasyanov <https://github.com/Unbewohnte>")
        .about("cowsay, but with soft friends")
        .arg(
            Arg::with_name("fumo")
            .short("f")
            .long("fumo")
            .value_name("fumofile")
            .help("Choose another fumofumo to print")
            .takes_value(true)
            .required(false)
            .default_value(FUMO_DEFAULT)
        )
        .arg(
            Arg::with_name("fumofiles_directory")
            .short("d")
            .long("fumofiles_directory")
            .value_name("fumofiles_directory_path")
            .help("Look for fumofiles in given directory")
            .takes_value(true)
            .required(false)
            .default_value(DEFAULT_FUMOFILES_DIR)
        )
        .arg(
            Arg::with_name("list_fumos")
            .short("l")
            .long("list_fumos")
            .help("Lists all fumofiles in the default directory instead")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("message")
            .short("m")
            .long("message")
            .help("Message to print")
            .takes_value(true)
            .index(1)
            .multiple(true)
        ).get_matches();

    // check for list_fumos flag.
    if matches.is_present("list_fumos") {
        // list fumofiles and exit
        let fumofiles = fs::read_dir(DEFAULT_FUMOFILES_DIR).expect("Could not read default fumofiles directory");
        for entry in fumofiles {
            let file = entry.unwrap();
            let filename = file.file_name();
            print!("{:?}\n", filename);
        }
        std::process::exit(0);
    }

    // process fumofiles directory
    
    // directory with all fumofiles
    let new_fumofiles_dir = matches.value_of("fumofiles_directory").unwrap();
    // name of the fumofile to work with
    let fumofile_name = matches.value_of("fumo").unwrap();
    // the whole path to the selected fumo
    let fumofile_path = Path::new(&new_fumofiles_dir).join(fumofile_name);
    
    // read fumofile
    let mut fumofile_contents: String = fs::read_to_string(fumofile_path).expect("Could not find a fumofile!");
    
    // process the message
    let message_values = matches.values_of("message").expect("No message provided");
    let message_vec: Vec<&str> = message_values.collect();
    let joined_message = message_vec.join(" ");

    // parse the file, insert fiven message and get the resulting string
    let fumosay: String = process_message(&mut fumofile_contents, joined_message.as_str());
    
    println!("{}", fumosay);
}


