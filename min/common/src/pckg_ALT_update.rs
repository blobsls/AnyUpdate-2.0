use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .arg("update")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("Packages updated successfully!");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Failed to update packages.");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
