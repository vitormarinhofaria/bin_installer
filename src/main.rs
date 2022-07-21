use std::fs::*;
#[cfg(target_os = "windows")]
use std::os::windows::fs::*;
#[cfg(target_os = "windows")]
fn create_symlink(source: &str, destination: &str) -> std::io::Result<()> {
    symlink_file(source, destination)
}
#[cfg(not(target_os = "windows"))]
use std::os::linux::fs::*;
#[cfg(not(target_os = "windows"))]
fn create_symlink(source: &str, destination: &str) -> std::io::Result<()> {
    symlink(source, destination)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        println!("Arg: {}", arg);
    }

    let destination = args
        .get(1)
        .expect("Usage: bin_installer [destination] [source]");
    let source = args
        .get(2)
        .expect("Usage: bin_installer [destination] [source]");
    let source_name = source
        .split("/")
        .last()
        .expect("Error: source file may be invalid.");

    std::fs::hard_link(source, format!("{}/{}", destination, source_name).as_str()).expect(
        format!(
            "Error: Failed to create shortcut to {} on {}.",
            source, destination
        )
        .as_str(),
    );
}
