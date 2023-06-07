use std::io::{self, Write};

use dialoguer::Confirm;

pub fn start_format(device: String, no_confirm: bool) {
    if !no_confirm {
        if Confirm::new()
            .with_prompt("Are you sure you want to format this disk?")
            .default(false)
            .interact()
            .unwrap()
        {
            println!("Type 'CTRL + C' to cancel operation");
            for i in (0..11).rev() {
                print!("FORMATTING {} IN -> {} \r", device, i);
                io::stdout().flush().unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            format_device(device);
        } else {
            println!("Operation cancelled");
        }
    } else {
        format_device(device);
    }

    println!("Exiting program");
    std::process::exit(0);
}

fn format_device(device: String) {
    println!("Formatting disk... Please be patient. This may take a while");
    zero_fill_disk(device);
    println!("Disk formatted successfully");
    println!("Please format disk with a filesystem of your choice");
}

fn zero_fill_disk(device: String) {
    let mut format_command = std::process::Command::new("dd");
    format_command
        .arg("if=/dev/zero")
        .arg("of=".to_owned() + &device)
        .arg("bs=1M")
        .spawn()
        .expect("Failed to format disk")
        .wait()
        .expect("Failed to format disk");
}
