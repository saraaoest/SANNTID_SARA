use std::process::Command; //Stdio
//use std::io::{Write, Read};
use std::thread::sleep;
use std::time::Duration;
use std::env;
use std::net::UdpSocket;

//start program:
// cargo run -- a

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        println!("{}", arg);
    }

    if args.len() == 2 {
        match args[1].as_str() {
            "a" => primary(),
            "b" => backup(),
            _ => println!("Invalid argument. Use 'a' or 'b'."),
        }
    } else {
        println!("Usage: cargo run -- (a|b)");
    }
}

fn primary() {
    //UBUBTU
    let command = "x-terminal-emulator";  
    let args = vec!["-e", "bash", "-c", "cargo run -- b; exec bash"];

    let mut child = Command::new(command)
        .args(args)
        //.stdin(Stdio::piped())  
        //.stdout(Stdio::piped()) 
        .spawn()
        .expect("Failed to launch terminal");
    
    
    //let socket = UdpSocket::bind("0.0.0.0:30000").expect("couldn't bind to address");
    //let socket = UdpSocket::bind("127.0.0.1:1234").expect("couldn't bind to address");
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    loop {
        // Create a buffer to receive data
        let mut buf = [0; 1024];

        // Receive data from the socket
        let (len, src) = socket.recv_from(&mut buf)?;

        // Print the received data and source address
        println!("Received {} bytes from {:?}", len, src);
        println!("Data: {:?}", String::from_utf8_lossy(&buf[..len]));
    }
    
    // {
    //     let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    //     stdin.write_all(b"hello world").expect("Failed to write to stdin");
    // } // `stdin` is closed here automatically

    // let mut output = String::new();
    // child.stdout.as_mut().expect("Failed to open stdout")
    //     .read_to_string(&mut output)
    //     .expect("Failed to read stdout");

    // let status = child.wait().expect("Failed to wait on child");
    // if status.success() {
    //     println!("Responded: {}", output);
    // } else {
    //     println!("Process failed");
    // }    

    let mut count = 0;
    loop{
        sleep(Duration::from_secs(1));
        count += 1;
        println!("{}", count);
        if count == 20 {
            panic!("Process crashed!");
        }
    }
}

fn backup(){
    println!("Backup mode activated.");
    sleep(Duration::from_secs(10));
}


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
    // let command = "cmd";  // Use cmd to start a new process
    // let args = vec!["/C", "start", "powershell", "-NoExit", "-Command", "cargo run main.rs b; Read-Host"];