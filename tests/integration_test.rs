use std::process::Command;

#[test]
fn test_wc_output_lines() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "-l", "test_files/test1.txt"])
        .output()
        .expect("Falha ao executar");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("       3 test1.txt")); 
}
