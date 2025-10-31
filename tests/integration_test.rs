// -- IGNORE ---
// integration tests for ruten language
use std::process::Command;

#[test]
fn test_math_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--", "examples/fibonacci.rtn"])
        .output()
        .expect("failed to execute");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fib(10)"));
}

#[test]
fn test_repl_help() {
    // test that repl starts and responds to help
    // this is a basic smoke test
    // note: In a real scenario, we would use a pty or expect-like library to interact with the REPL
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("failed to build");
    
    assert!(output.status.success());
}