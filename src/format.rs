use std::io::{self, Write};

use dialoguer::Confirm;

pub fn start_format(device: String) {

    if Confirm::new()
        .with_prompt("Are you sure you want to format this disk?")
        .default(false)
        .interact()
        .unwrap()
    {
        // wait for 5 second and display coundown to user using a for loop
        println!("Type 'CTRL + C' to cancel operation");
        for i in (0..11).rev() {
            print!("FORMATTING {} IN -> {}\r", device, i);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        zero_fill_disk(device);
        println!("Disk formatted successfully");
    } else {
        println!("Exiting program");
        std::process::exit(0);
    }
        
}

fn zero_fill_disk(device: String) {
    sudo::escalate_if_needed().unwrap();
    std::process::Command::new("cat")
        .arg("/dev/zero")
        .arg(">")
        .arg(device)
        .spawn()
        .expect("Failed to execute command");
}