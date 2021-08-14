mod fumo;

use std::path::Path;
use std::fs;
use std::env;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();

    // try to retrieve a message from args
    if args.len() <= 1 {
        // no message was provided
        std::process::exit(1);
    }
    
    let clarg_message = &args[1..].join(" ");

    // path to the fumofile
    let fumofile_default_path = format!("./fumofiles/{}",fumo::FUMO_DEFAULT);
    let fumofile_path = Path::new(&fumofile_default_path);

    // read fumofile
    let mut fumofile_contents = fs::read_to_string(fumofile_path).unwrap();
    
    // parse the file and get the resulting string
    let message: String = fumo::sayify(&mut fumofile_contents, clarg_message);
    
    println!("{}", message);
}


