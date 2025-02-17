use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::thread::sleep;
use std::time::Duration;
use std::env;

//start program:
// cargo run main.rs -- a

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        println!("{}", args[i]); // Use {} for string interpolation
    }

    if args.len() == 3 {
        if args[2] == "a" {
            primary();
        } else if args[2] == "b" {
            backup();
        }
    } else {
        println!("Invalid arguments.");
    }
    
}

fn primary() {
    
    // Command::new("x-terminal-emulator")  // Opens a new terminal
    //     .arg("-e")
    //     .arg(format!("bash -c '{}'", script)) // Runs script inside a new terminal
    //     .spawn()
    //     .expect("Failed to launch terminal");

    //WSL
    // to open new wsl terminal (cmd.exe /c start cmd.exe /c wsl.exe) or (cmd.exe /c start bash)
    //doesn't work: 
    //cmd.exe /c start bash cargo run main.rs b
    //opens and closes at once
    // Command::new("cmd.exe")
    //         .arg("/c")
    //         .arg("start")
    //         .arg("bash")
    //         //.arg("-i")
    //         //.arg("-c")
    //         //.arg("cd 'Exercise_1/shared_variable/c' && ./foo.exe")
    //         //.arg("cd '/mnt/c/Users/sarao/OneDrive\\ -\\ NTNU/Documents/KYB/kyb6/SANNTID/Exercise_1/shared_variable/c' && ./foo.exe && echo 'Press any key to exit...' && read")
    //         .spawn()
    //         .expect("Failed to launch terminal");

    //Powershell
    let command = "cmd";  // Use cmd to start a new process
    let args = vec!["/C", "start", "powershell", "-NoExit", "-Command", "cargo run main.rs b; Read-Host"];

    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())  
        .stdout(Stdio::piped()) 
        .spawn()
        .expect("Failed to launch terminal");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(b"hello world").expect("Failed to write to stdin");
    } // `stdin` is closed here automatically

    let mut output = String::new();
    child.stdout.as_mut().expect("Failed to open stdout")
        .read_to_string(&mut output)
        .expect("Failed to read stdout");

    let status = child.wait().expect("Failed to wait on child");
    if status.success() {
        println!("Responded: {}", output);
    } else {
        println!("Process failed");
    }    

    // let mut count = 0;
    // loop{
    //     sleep(Duration::from_secs(1));
    //     count += 1;
    //     println!("{}", count);
    //     if count == 20 {
    //         panic!("Process crashed!");
    //     }
    // }
}

fn backup(){
    println!("Backup mode activated.");
}
