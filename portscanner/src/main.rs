use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use std::env;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::time::timeout;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]  
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: {} <ip> <1..5000>", args[0]);
        return;
    }
    let ip = &args[1];
    let port_range = args[2].split("..").collect::<Vec<&str>>();
    if port_range.len() != 2 {
        println!("Invalid port range");
        return;
    }
    let start_port = u16::from_str(port_range[0]).expect("Invalid port number");
    let end_port = u16::from_str(port_range[1]).expect("Invalid port number");
    let start_time = Instant::now();
    if start_port > end_port {
        eprintln!("Start port must be less than or equal to end port");
        return;
    }

    /*for port in start_port..=end_port {
        if scan_port(ip, port) {
            println!("Port {} is open", port)
        } else {
            println!("Port {} is closed", port);
        }

    }*/
    /*  threaded_scan(ip, start_port, end_port);*/
    //tokio_scan_port(ip, start_port, end_port).await;
    semaphore_scan(ip, start_port, end_port).await;
    let elapsed_time = start_time.elapsed();
    println!("Scan completed in {} seconds", elapsed_time.as_secs());
}

async fn scan_port_async(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port)
        .parse::<SocketAddr>()
        .expect("Invalid IP address");
    timeout(Duration::from_secs(1), TokioTcpStream::connect(&address))
        .await
        .is_ok()
}

fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port)
        .parse::<SocketAddr>()
        .expect("Invalid IP address");
    TcpStream::connect_timeout(&address, Duration::from_secs(1)).is_ok()
}

async fn tokio_scan_port(ip: &str, start_port: u16, end_port: u16) {
    let mut tasks = vec![];
    for port in start_port..=end_port {
        let ip = ip.to_string();
        let task = tokio::spawn(async move {
            if scan_port_async(&ip, port).await {
                println!("Port {} is open", port)
            } else {
                println!("Port {} is closed", port);
            }
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await.unwrap();
    }
}

async fn semaphore_scan(ip: &str, start_port: u16, end_port: u16) {
    let mut tasks = vec![];
    let max_concurrent = 1000; // Limit to 10 simultaneous connections
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    for port in start_port..=end_port {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let ip = ip.to_string();
        let task = tokio::spawn(async move {
            if scan_port_async(&ip, port).await {
                println!("Port {} is open", port)
            } else {
                println!("Port {} is closed", port);
            }
            drop(permit);
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await.unwrap();
    }
}

fn threaded_scan(ip: &str, start_port: u16, end_port: u16) {
    let mut handles = vec![];
    for port in start_port..=end_port {
        let ip = ip.to_string();
        let handle = thread::spawn(move || {
            if scan_port(&ip, port) {
                println!("Port {} is open", port)
            } else {
                println!("Port {} is closed", port);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn threaded_scan_safe(ip: &str, start_port: u16, end_port: u16) {
    let start = Instant::now();
    let mut handles = Vec::new();
    for port in start_port..=end_port {
        let ip = ip.to_string();
        let handle = thread::Builder::new()
            .stack_size(64 * 1024) // 64KB instead of 2MB
            .spawn(move || {
                if scan_port(&ip, port) { println!("Port {} is open", port); }
                else { println!("Port {} is closed", port); }
            })
            .unwrap();
        handles.push(handle);
    }
    for handle in handles { handle.join().unwrap(); }
    println!("Threaded completed in {:?}", start.elapsed());
}
