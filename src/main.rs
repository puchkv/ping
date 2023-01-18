
use std::env;
use std::thread;
use std::time::Duration;
use http::{ Method, StatusCode };

fn main() {

    let args: Vec<String>= env::args().collect();

    let timeout = args[1].parse().unwrap();
    let domain = args[2].clone();

    ping(timeout, &domain);
}

fn ping(timeout: u64, addr: &str) {

    let client = reqwest::blocking::Client::new(); // это не круто

    let response = client
        .request(Method::GET, addr)
        .timeout(Duration::from_secs(timeout))
        .send();

    match response {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => {
                    println!("Addr: {}, Status: {}", resp.url(), resp.status());
                }
                _ => {
                    println!("Addr: {}, Status: {}", resp.url(), resp.status());
                    println!("Trying again...");
                    thread::sleep(Duration::from_secs(timeout));
                    ping(timeout, addr);
                }
            }
        }
        Err(error) => {
            println!("[ERROR]: {}", error);
        }
    }
    ()
}
