use std::{fs::File, io::Write, process::exit};
use text_io::read;

fn main() {
    let username: String = whoami::username();
    println!("Enter a name of the service you're adding. This will be the name of the .service file and should be unique");
    let name: String = get_input(false, false);
    println!("Enter a description for the service. (Optional, press Enter to skip)");
    let description: String = get_input(true, true);
    println!(
        "Enter the command that should be executed to run your service. For example: /bin/java -jar /home/user/myapp.jar"
    );
    let exec: String = get_input(false, true);
    println!("Enter a working directory, this is useful when storing logs/files (Optional, press Enter to skip)");
    let working_dir: String = get_input(true, false);
    println!("Enter a username that runs the process, should not be root. (Default: {}, press Enter to accept.)", username);
    let mut username_input = get_input(true, false);
    if username_input.is_empty() {
        username_input = username;
    }

    let unit_string: String;
    if description.is_empty() {
        unit_string = "[Unit]\nAfter=multi-user.target\nAfter=network-online.target\nWants=network-online.target".to_string();
    } else {
        unit_string = format!(
            "[Unit]\nDescription={}\nAfter=multi-user.target\nAfter=network-online.target\nWants=network-online.target",
            description
        );
    }

    let service_string: String;
    if working_dir.is_empty() {
        service_string = format!(
            "[Service]\nExecStart={}\nUser={}\nType=idle\nRestart=always\nRestartSec=10\nRestartPreventExitStatus=0\nTimeoutStopSec=10\n",
            exec, username_input
        );
    } else {
        service_string = format!(
            "[Service]\nExecStart={}\nUser={}\nType=idle\nRestart=always\nRestartSec=10\nRestartPreventExitStatus=0\nTimeoutStopSec=10\nWorkingDirectory={}\n\n",
            exec, username_input, working_dir
        );
    }

    let final_string = String::from(
        unit_string + "\n" + &service_string + "\n[Install]\nWantedBy=multi-user.target",
    );
    let filename = format!("/etc/systemd/system/{}.service", name);

    let file = File::create(&filename);
    if file.is_err() {
        println!(
            "There was an error while creating the new file: {:?}",
            file.err()
        )
    } else {
        let result = file.unwrap().write_all(final_string.as_bytes());
        if result.is_err() {
            println!(
                "There was an error while writing to the new file: {:?}",
                result.err()
            );
        } else {
            exit(0);
        }
    }
    println!(
        "You can run the following command to create the file with the contents:\necho \"{}\" | sudo tee -a {}",
        final_string, filename
    )
}

fn get_input(can_be_empty: bool, can_contain_whitespaces: bool) -> String {
    loop {
        let input: String = read!("{}\n");
        if input.is_empty() && !can_be_empty {
            println!("This can't be empty, please try again.");
        } else if input.contains(" ") && !can_contain_whitespaces {
            println!("This can't contain whitespaces, please try again.")
        } else {
            return input;
        }
    }
}
