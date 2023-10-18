#[cfg(test)]
mod tests;

use clap::{App, Arg};
use std::error::Error;
use std::process::{Command, Stdio};
use std::str;
use std::io;

// Generic Result type that allows for errors
type GenResult<T> = Result<T, Box<dyn Error>>;

// Struct to hold the arguments
#[derive(Debug)]
pub struct Config {
    desktop_env: Vec<String>,
    has_nvidia: bool,
    nvidia_vers: String,
    auto_reboot: bool,
}

// Sets up the arguments that can be passed and puts them into the Config struct
pub fn get_args() -> GenResult<Config> {
    let matches = App::new("ublue_it_cli")
        .version("0.1.0")
        .author("Bryan Hyland <bryan.hyland32@gmail.com>")
        .about("Rust CLI for ublue.it distro tool for Fedora Silverblue and Fedora Kinoite.")
        .arg(
            Arg::with_name("desktop_env")
            .short("d")
            .long("desktop_env")
            .help("Choices are Bluefin (Silverblue for Ubuntu Ex-Pats), Gnome (default), KDE, LXQt, Mate, or XFCE.\n*NOTE: If KDE is chosen, Kinoite will be the base OS image instead of Silverblue.")
            .takes_value(true)
            .default_value("gnome"),
        )
        .arg(
            Arg::with_name("has_nvidia")
            .short("n")
            .long("has_nvidia")
            .help("Use this flag if your system has a Nvidia GPU.")
            .takes_value(false),
        )
        .arg(
            Arg::with_name("nvidia_vers")
            .long("nvidia_vers")
            .help("Use this flag to specify the version of the Nvidia driver you want to install.\nDefault is the latest version.\n
                Choices are 470, 525, current, latest.")
            .takes_value(true)
            .default_value("")
        )
        .arg(
            Arg::with_name("auto_reboot")
            .short("r")
            .long("auto-reboot")
            .help("Do you want your system to reboot automatically when the process is complete?\nDefault is that this is manual.")
            .takes_value(false),
        )
        .get_matches();

    let desktop_env = matches.values_of_lossy("desktop_env").unwrap();
    let has_nvidia = matches.is_present("has_nvidia");
    let nvidia_vers = matches.value_of("nvidia_vers").unwrap().to_string();
    let auto_reboot = matches.is_present("auto_reboot");

    Ok(Config {
        desktop_env,
        has_nvidia,
        nvidia_vers,
        auto_reboot,
    })
}

// The main logic
pub fn run(config: Config) -> GenResult<()> {
    // Set the desktop_env to lowercase in the case it wasn't already
    let de: String = config.desktop_env[0].clone().to_lowercase();

    // Rebase the image
    rebase_img(de, config.has_nvidia, config.nvidia_vers, config.auto_reboot);

    Ok(())
}

// Uses shell commands cat and cut to get the version of Fedora the user is on
fn get_fedora_version() -> String {
    let fedora_release = Command::new("cat")
        .arg("/etc/fedora-release")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let get_version = Command::new("cut")
        .arg("-d ")
        .arg("-f3")
        .stdin(Stdio::from(fedora_release.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = get_version.wait_with_output().unwrap();
    let fedora_version = str::from_utf8(&output.stdout).unwrap();

    fedora_version.to_string()
}

fn get_nvidia_vers(nvidia_vers: String) -> String {
    let mut nvers: String = nvidia_vers.clone();
    loop {
        match nvers.to_lowercase().as_str() {
            "470"| "525" | "current" => {
                nvers = nvers.to_lowercase();
            }
            "latest" | "" => {
                nvers = String::new();
            }
            _ => {
                eprintln!("You entered an incorrect version of the Nvidia driver!\nTry again!");
                nvers.clear();
                io::stdin().read_line(&mut nvers).unwrap_or(1);
                continue;
            }
        }
    }
}

fn get_img_name(dsk_env: String, is_nvidia: bool) -> String {
    let mut img_env: String;
    
    match dsk_env.as_str() {
        "bluefin" => {
            img_env = "bluefin".to_string();
        }
        "gnome" => {
            img_env = "silverblue".to_string();
        }
        "lxqt" | "mate "=> {
            img_env = dsk_env;
        }
        "xfce" => {
            img_env = "vauxite".to_string();
        }
        "kde" => {
            img_env = "kinoite".to_string();
        }
        _ => {
            eprintln!("You entered an incorrect desktop environment!\nTry again!");
            std::process::exit(1);
        }
    }

    if !is_nvidia && img_env != "bluefin" {
        img_env.push_str("-main");
    } else if is_nvidia {
        img_env.push_str("-nvidia");
    }

    img_env
}

fn create_cmd_string(dsk_env: String, is_nvidia: bool, nvidia_vers: String) -> String {
    // Get the version of Fedora
    let mut vers = get_fedora_version();
    // Get the proper image name we're rebasing to
    let img_env = get_img_name(dsk_env, is_nvidia);

    // If the user specified a version of the Nvidia driver to install
    let nvidia_vers = get_nvidia_vers(nvidia_vers);

    if !nvidia_vers.is_empty() {
        // If the user specified a version of the Nvidia driver to install
        vers = format!("{}-{}", vers, nvidia_vers);
    }
    
    // Format the version and image name together into on string and return it
    format!("ostree-unverified-registry:ghcr.io/ublue-os/{}:{}", img_env, vers)
}

// Installs the image with any valid desktop environment and version (including nvidia)
fn rebase_img(dsk_env: String, is_nvidia: bool, nvidia_vers: String, restart: bool) {
    println!("Installing, please be patient...");

    // Create the command string
    let cmd_str = create_cmd_string(dsk_env, is_nvidia, nvidia_vers);

    // Run the command in the bash shell
    let install_process = Command::new("bash")
        .args(&["-c", format!("rpm-ostree rebase --experimental {}", cmd_str).as_str()])
        .status()
        .expect("Could not start the rebase process...\nTry again!");

    // If the rebase process was successful
    if install_process.success() {
        if is_nvidia {
            // If it's a Nvidia image set the kargs
            let kargs_success = set_kargs(true);
            if kargs_success {
                // If setting the kargs is successful reboot if the flag is true
                reboot_computer(restart);
            }
        } else {
            // Reboot if the flag is true
            reboot_computer(restart);
        }
    } else {
        // Print an error with the install process status
        println!("Rebasing failed with status: {}", install_process);
    }
}

fn set_kargs(do_kargs_set: bool) -> bool {
    // Set kargs after the rebase process
    if do_kargs_set {
        // Remove the kargs if they already exist (prevents double kargs)
        Command::new("rpm-ostree")
            .arg("kargs")
            .arg("--delete-if-exists=rd.driver.blacklist=nouveau")
            .arg("--delete-if-exists=modprobe.blacklist=nouveau")
            .arg("--delete-if-exists=nvidia-drm.modeset=1")
            .status()
            .expect("Could not cleanup kargs...");

        // Set the kargs options
        let kargs = Command::new("rpm-ostree")
            .arg("kargs")
            .arg("--append=rd.driver.blacklist=nouveau")
            .arg("--append=modprobe.blacklist=nouveau")
            .arg("--append=nvidia-drm.modeset=1")
            .status()
            .expect("Could not set kargs...");

        return kargs.success();
    }

    false
}

// Reboot the computer
fn reboot_computer(do_reboot: bool) {
    if do_reboot {
        println!("{}", "Rebooting...");
        
        // reboot the computer
        let _reboot = Command::new("systemctl")
            .arg("reboot")
            .spawn()
            .expect("Could not reboot the computer, manually reboot for changes to take effect!")
            .wait()
            .expect("Could not reboot the computer, manually reboot for changes to take effect!");
    } else {
        println!("Your computer is ready to be rebooted manually using systemctl reboot!");
    }
}
