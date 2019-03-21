use std::process;
use std::process::{Command, Stdio};

fn main() {
    // output
    let output = Command::new("echo")
                         .arg("Hello world")
                         .output()
                         .expect("Failed to execute command");
    assert_eq!(b"Hello world\n", output.stdout.as_slice());

    // spawn
    let echo_child = Command::new("echo")
                             .arg("Oh no, a tpyo!")
                             .stdout(Stdio::piped())
                             .spawn()
                             .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    // Stdio
    let mut sed_child = Command::new("sed")
                                .arg("s/tpyo/typo/")
                                .stdin(Stdio::from(echo_out))
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("Failed to start sed process");

    // wait_with_output
    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());

    // id
    println!("process id: {}", process::id());

    // exit
    process::exit(-1);
}
