
#[cfg(target_os = "windows")]
const EXE_EXTENSION: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const EXE_EXTENSION: &str = "";

fn main(){
    let mut clang_comp = std::process::Command::new("clang++");
    clang_comp.args(["executable/executable.cpp", "-o", format!("executable/executable{}", EXE_EXTENSION).as_str()]);
    let _ = clang_comp.spawn();
}