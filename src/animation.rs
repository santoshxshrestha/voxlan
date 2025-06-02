use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;
pub fn show_pulsing() {
    let states = [
        "Server Status: .....",
        "Server Status: .....",
        "Server Status: READY",
        "Server Status: .....",
        "Server Status: READY",
        "Server Status: LIVE!",
    ];

    for state in &states {
        print!("\r{}", state);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(400));
    }
    println!();
}

pub fn start_spinner() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let spinner_chars = ['|', '/', '-', '\\'];
        loop {
            for &ch in &spinner_chars {
                print!("\rserver is running {} ", ch);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        }
    })
}
