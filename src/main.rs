use std::fs;
use std::env;
use clap::{Arg, App}; 

/// Indicator of where the message should be in fumofile
pub const MESSAGE_INDICATOR: &str = "!message";
/// Fumofile name of the default fumo
pub const FUMO_DEFAULT: &str = "fumo.fumo"; 

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
        .version("0.2.0")
        .author("Unbewohnte | Nikolay Kasyanov <https://github.com/Unbewohnte>")
        .about("cowsay, but with soft friends")
        .arg(
            Arg::with_name("fumo")
            .short("f")
            .long("fumo")
            .value_name("fumofile")
            .help("choose another fumofumo to print")
            .takes_value(true)
            .required(false))
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

    // default path to the fumofile

    // grab path to exe
    let executable_path = env::current_exe().expect("Could not get current exe`s path!");
    // remove the last bit (exe filename)
    let executable_dir = executable_path.parent().expect("Could not get exe`s parent directory !");
    // local path to the default fumofile
    let fumofile_default_path = std::path::Path::new("fumofiles").join(FUMO_DEFAULT);
    // add them together
    let fumofile_path = executable_dir.join(fumofile_default_path);

    // read fumofile
    let mut fumofile_contents: String = fs::read_to_string(fumofile_path).expect("Could not find a fumofile!");
    
    // parse the file and get the resulting string
    let message: String = process_message(&mut fumofile_contents, matches.value_of("message").unwrap());
    
    println!("{}", message);
}


