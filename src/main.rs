mod scrape;

fn main() {

    println!("[ + ] Starting ........");

    println!("[ + ] Create a directory for datasets");
    
    std::process::Command::new("cmd")
        .args(&["/C", "mkdir datasets"])
        .output()
        .expect("failed to execute process");

    if let Err(error_message) = scrape::smp::get_school_profile(
        22010000, 290
    ) {
        eprintln!("{}", error_message);
    }
    
}