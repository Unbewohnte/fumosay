use std::path::Path;
use std::fs;
use std::env;

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
    let args: Vec<String> = env::args().collect();

    // try to retrieve a message from args
    if args.len() <= 1 {
        // no message was provided
        std::process::exit(1);
    }
    
    let clarg_message: &String = &args[1..].join(" ");

    // path to the fumofile
    let fumofile_default_path: String = format!("./fumofiles/{}",FUMO_DEFAULT);
    let fumofile_path: &Path = Path::new(&fumofile_default_path);

    // read fumofile
    let mut fumofile_contents: String = fs::read_to_string(fumofile_path).unwrap();
    
    // parse the file and get the resulting string
    let message: String = process_message(&mut fumofile_contents, clarg_message);
    
    println!("{}", message);
}


