use winers::Wine;

fn main() {
    let prefix_path = "/home/missrce/.local/share/rumester/wine"; // prefix these
    let prefix = Wine::new(&prefix_path);

    match prefix.init() {
        Ok(_) => println!("Wine prefix initialized successfully at '{}'.", prefix_path),
        Err(e) => println!("Error: {}", e)
    };

    let output = prefix.cmd()
        .arg("notepad")
        .output()
        .expect("how the hell did notepad fail to run");

    println!("status: {}", output.status);

    match prefix.delete() {
        Ok(_) => println!("Wine prefix deleted successfully at '{}'.", prefix_path),
        Err(e) => print!("Error: {}", e.to_string())
    };
}