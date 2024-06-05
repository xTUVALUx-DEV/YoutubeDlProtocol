use std::{env, path::Path, env::home_dir};
use std::process::Command;
use notify_rust::{Notification, Timeout};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = &args[1];
        handle_url(url);
    } else {
        eprintln!("No URL provided");
    }
}

fn handle_url(url: &str) {
    if url.starts_with("ydl://") {
        assert!(env::set_current_dir(format!("{}/Downloads", env::var("USERPROFILE").unwrap())).is_ok());

        let resource = &url[6..]; // Remove 'ydl://'
        let url = "https://".to_string() + resource;
        if resource.contains("you"){
            let _ = Command::new("youtube-dl.exe").args([url]).spawn();
            show_notification("", &format!("Handling custom URL protocol: ydl://{}", resource));
            
        }
        // Here you can add your custom handling logic
    } else {
        eprintln!("Invalid URL protocol");
    }
}

fn show_notification(title: &str, message: &str) {
    Notification::new()
        .summary(title)
        .body(message)
        .icon("dialog-information")
        .sound_name("chord")
        .timeout(Timeout::Milliseconds(4000))
        .show()
        .expect("Failed to display notification");
}