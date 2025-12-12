use expanduser::expanduser;
use std::fs::{self, copy, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::process::Command;
use text_io::read;

fn config_dir() -> String {
    let config_dir = expanduser("~/.config/pipewire/pipewire.conf.d/")
        .expect("This should never fail.")
        .into_os_string()
        .into_string()
        .unwrap();
    config_dir
}

fn get_config_name(default_config_path: String) -> String {
    let file = match File::open(&default_config_path) {
        Ok(f) => f,
        Err(_) => return String::from("Unknown"),
    };

    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if line.contains("media.name") {
                if let Some(name_part) = line.split("= ").nth(1) {
                    return name_part.replace('"', "");
                }
            }
        }
    }
    String::from("Unknown")
}

fn list_configs(config_dir: String) -> Vec<String> {
    println!("Available configurations:");
    let paths_result = fs::read_dir(&config_dir);
    if paths_result.is_err() {
        eprintln!("Error: Could not read config directory. Does ~/.config/pipewire/pipewire.conf.d/ exist?");
        process::exit(1);
    }

    let mut paths: Vec<_> = paths_result
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    paths.sort_by_key(|dir| dir.path());
    let mut all_confs = Vec::<String>::new();
    let mut counter: i8 = 1;
    for path in paths {
        let file_path = path.path();
        if let Some(file_name) = file_path.file_name() {
            let file_name_str = file_name.to_string_lossy();
            // Only include files WITHOUT .conf extension
            if Path::new(&file_name_str.as_ref()).extension().is_none() {
                all_confs.push(file_name_str.to_string());
                println!("{}: {}", counter, get_config_name(file_path.to_string_lossy().to_string()));
                counter += 1;
            }
        }
    }
    println!("{}: (Q)uit", counter);
    return all_confs;
}

fn select_config(all_confs: Vec<String>) -> usize {
    println!("Enter selection: ");
    let input_text: String = read!();
    if input_text.to_lowercase() == "q" {
        process::exit(0x0100);
    } else {
        let validity_check = input_text.parse::<usize>();

        match validity_check {
            Ok(value) => {
                if value == 0 || value > all_confs.len() {
                    println!("Selection out of range. Please choose a number between 1 and {}.", all_confs.len());
                    return select_config(all_confs);
                }
                return value - 1;
            }
            Err(_err) => {
                println!("You did not enter a valid value.");
                return select_config(all_confs);
            }
        }
    }
}

fn replace_config(config_dir: String, default_config_path: String, selection: String) {
    let selection_path = Path::new(&config_dir).join(&selection);
    if let Err(e) = copy(&selection_path, &default_config_path) {
        eprintln!("Error: Could not copy configuration file: {}", e);
        process::exit(1);
    }
}

fn reload_pipewire() {
    let pipewire_restart = Command::new("systemctl")
        .arg("--user")
        .arg("restart")
        .arg("pipewire")
        .status();
    let mut flag1 = false;
    let pipewire_pulse_restart = Command::new("systemctl")
        .arg("--user")
        .arg("restart")
        .arg("pipewire-pulse")
        .status();
    let mut flag2 = false;
    match pipewire_restart {
        Ok(status) => {
            if status.success() {
                flag1 = true;
            } else {
                eprintln!("Failed to restart pipewire (exit code: {:?})", status.code());
            }
        }
        Err(e) => eprintln!("Could not execute systemctl for pipewire: {}", e),
    }
    match pipewire_pulse_restart {
        Ok(status) => {
            if status.success() {
                flag2 = true;
            } else {
                eprintln!("Failed to restart pipewire-pulse (exit code: {:?})", status.code());
            }
        }
        Err(e) => eprintln!("Could not execute systemctl for pipewire-pulse: {}", e),
    }
    if flag1 && flag2 {
        println!("Successfully restarted pipewire and pipewire-pulse!");
    }
}

fn main() {
    let default_config_name = String::from("sink-eq6.conf");
    let config_dir = config_dir();
    let default_config_path = Path::new(&config_dir)
        .join(&default_config_name)
        .to_string_lossy()
        .to_string();
    println!(
        "Current config: {}",
        get_config_name(default_config_path.clone())
    );
    let all_confs = list_configs(config_dir.clone());
    let selection_number = select_config(all_confs.clone());
    let selection = &all_confs[selection_number];
    replace_config(
        config_dir.clone(),
        default_config_path.clone(),
        selection.clone().to_string(),
    );
    reload_pipewire()
}
