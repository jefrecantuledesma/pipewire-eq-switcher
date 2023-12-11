import os
import shutil
import subprocess

CONFIG_DIR = os.path.expanduser("~/.config/pipewire/pipewire.conf.d")
DEFAULT_CONFIG_NAME = "sink-eq6.conf"

def list_configs():
    print("Available configurations:")
    configs = [f for f in os.listdir(CONFIG_DIR) if f.endswith('.conf')]
    for i, config in enumerate(configs, start=1):
        config = config.replace(".config", "")
        print(f"{i}. {config}")

def select_config():
    list_configs()
    choice = input("Enter the number of the configuration you want to use: ")
    return choice

def replace_default_config(selected_config):
    default_config_path = os.path.join(CONFIG_DIR, DEFAULT_CONFIG_NAME)
    selected_config_path = os.path.join(CONFIG_DIR, selected_config)

    try:
        os.remove(default_config_path)
        os.rename(selected_config_path, default_config_path)
        shutil.copy(default_config_path, selected_config_path)
        print("Configuration updated successfully!")
    except FileNotFoundError:
        print("Error: File not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

def restart_services():
    try:
        subprocess.run(["systemctl", "--user", "restart", "pipewire"], check=True)
        subprocess.run(["systemctl", "--user", "restart", "pipewire-pulse"], check=True)
        print("Pipewire and Pulseaudio restarted successfully!")
    except subprocess.CalledProcessError as e:
        print(f"Error restarting services: {e}")

if __name__ == "__main__":
    selected_config_index = int(select_config()) - 1
    configs = [f for f in os.listdir(CONFIG_DIR) if f.endswith('.conf')]

    try:
        selected_config = configs[selected_config_index]
        replace_default_config(selected_config)
        restart_services()
    except IndexError:
        print("Invalid choice. Please enter a valid number.")

