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
            print!("FORMATTING {} IN -> {} \r", device, i);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Formatting disk... Please be patient. This may take a while");
        zero_fill_disk(device);
        println!("Disk formatted successfully");
        println!("Please format disk with a filesystem of your choice");
    } else {
        println!("Operation cancelled");
    }

    println!("Exiting program");
    std::process::exit(0);
}

fn zero_fill_disk(device: String) {
    let mut format_command = std::process::Command::new("cat");
    format_command.arg("/dev/zero").arg(">").arg(device);

    format_command
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");
}
