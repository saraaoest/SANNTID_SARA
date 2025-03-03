use std::process::Command; 
use std::thread::sleep;
use std::time::Duration;
use std::env;
use std::net::UdpSocket;
use std::thread;

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
        println!("Usage: cargo run --(a/b)");
    }

   

    // let handler1 = thread::spawn(|| {
    //     let mut i = 1;
    //     loop {
    //         primary();
    //         // println!("hi number {i} from the spawned thread!");
    //         // i = i + 1;
    //         thread::sleep(Duration::from_secs(10));
            
    //     }
    // });

    // let handler2 = thread::spawn(|| {
    //     let mut i = 1;
    //     loop{
    //         backup()
    //         // println!("hi number {i} from the main thread!");
    //         // i = i + 1;
    //         thread::sleep(Duration::from_secs(10));
    //     }
    // });    
    
    // handler1.join().unwrap();
    // handler2.join().unwrap();

    

}

fn primary() {
    //UBUBTU
    let command = "x-terminal-emulator";  
    let args = vec!["-e", "bash", "-c", "cargo run -- b; exec bash"];

    Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to launch terminal");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind primary socket"); // Use an ephemeral port
    socket.set_broadcast(true).expect("Couldn't set broadcast");
    
    let id = "255.255.255.255:1234";
    //let id = "127.0.0.1:1234"; // last must not be over ca. 3000 
    //let socket = UdpSocket::bind("0.0.0.0:30000").expect("couldn't bind to address");
    //let socket = UdpSocket::bind(id).unwrap();
    //let mut count: [u8; 10] = [0; 10]; //array of 10 bytes

    let message = b"Hello from Primary";

    loop{
        //socket.send_to(&count, id).expect("couldn't send data");
        socket.send_to(message, id).expect("couldn't send data");
        println!("Primary sent: {}", String::from_utf8_lossy(message));
        sleep(Duration::from_secs(10));
    }
}

fn backup(){
    println!("Backup mode activated.");

    let socket = UdpSocket::bind("0.0.0.0:1234").unwrap(); // must listen on 0.0.0.0
    let mut buf: [u8; 10] = [0; 10];
    loop {
        let (len, src) = socket.recv_from(&mut buf).expect("Didn't receive data"); // Receive data from the socket

        // Print the received data and source address
        // println!("Received {} bytes from {:?}", len, src);
        // println!("Data: {:?}", String::from_utf8_lossy(&buf[..len]));
        println!("Backup received '{}' from {}", String::from_utf8_lossy(&buf[..len]), src);
        sleep(Duration::from_secs(1));
    }
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