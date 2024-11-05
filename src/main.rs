use expanduser::expanduser;
use std::fs::{self, copy, File};
use std::io::{BufRead, BufReader};
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
    let file = File::open(&default_config_path).expect("Couldn't read file. Does it exist?");
    let mut target_line = String::new();

    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        if line.contains("media.name") {
            target_line = line;
            break;
        }
    }
    let parts = target_line.split("= ");
    let target_conf = parts.collect::<Vec<&str>>()[1];
    target_conf.replace('"', "")
}

fn list_configs(config_dir: String) -> Vec<String> {
    println!("Available configurations:");
    let mut paths: Vec<_> = fs::read_dir(config_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());
    let mut all_confs = Vec::<String>::new();
    let mut counter: i8 = 1;
    for path in paths {
        let path_str = path.path().into_os_string().into_string().unwrap();
        if let Some(index) = path_str.clone().rfind("/") {
            let file_name = &path_str[(index + 1)..];
            if !file_name.contains(".conf") {
                all_confs.push(file_name.to_string());
                println!("{}: {}", counter, get_config_name(path_str));
                counter += 1;
            }
        };
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
            Ok(value) => return value - 1,
            Err(_err) => {
                println!("You did not enter a valid value.");
                return select_config(all_confs);
            }
        }
    }
}

fn replace_config(config_dir: String, default_config_path: String, selection: String) {
    let selection_path: String = config_dir + &selection;
    copy(selection_path, default_config_path).expect("Could not copy :(");
}

fn reload_pipewire() {
    let pipewire_restart = Command::new("systemctl")
        .arg("--user")
        .arg("restart")
        .arg("pipewire")
        .spawn();
    let mut flag1 = false;
    let pipewire_pulse_restart = Command::new("systemctl")
        .arg("--user")
        .arg("restart")
        .arg("pipewire-pulse")
        .spawn();
    let mut flag2 = false;
    match pipewire_restart {
        Ok(_val) => flag1 = true,
        Err(_err) => println!("Did not successfully restart pipewire."),
    }
    match pipewire_pulse_restart {
        Ok(_val) => flag2 = true,
        Err(_err) => println!("Did not successfully restart pipewire-pulse."),
    }
    if flag1 && flag2 {
        println!("Successfully restarted pipewire and pipewire-pulse!");
    }
}

fn main() {
    let default_config_name = String::from("sink-eq6.conf");
    let config_dir = config_dir();
    let default_config_path: String = config_dir.clone() + &default_config_name;
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
