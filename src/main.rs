mod fumo;
fn main() {
    let mut fumofile_contents = std::fs::read_to_string(std::path::Path::new("./fumofiles/fumo.fumo")).unwrap();
    let message: String = fumo::sayify(&mut fumofile_contents,"fumo");
    println!("{}", message);
}


