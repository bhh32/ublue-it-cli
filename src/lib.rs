use clap::{App, Arg};
use std::error::Error;
use std::process::{Command, Stdio};
use std::str;

// Generic Result type that allows for errors
type GenResult<T> = Result<T, Box<dyn Error>>;

// Struct to hold the arguments
#[derive(Debug)]
pub struct Config {
    desktop_env: Vec<String>,
    has_nvidia: bool,
    auto_reboot: bool,
}

// Sets up the arguments that can be passed and puts them into the Config struct
pub fn get_args() -> GenResult<Config> {
    let matches = App::new("ublue_it_cli")
        .version("0.1.0")
        .author("Bryan Hyland <bryan.hyland32@gmail.com")
        .about("Rust CLI for ublue.it distro tool for Fedora Silverblue and Fedora Kinoite.")
        .arg(
            Arg::with_name("desktop_env")
            .short("d")
            .long("desktop_env")
            .help("Choices are Gnome (default), KDE, LXQt, Mate, or XFCE.\n*NOTE: If KDE is chosen, Kinoite will be the base OS image instead of Silverblue.")
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
            Arg::with_name("auto_reboot")
            .short("r")
            .long("auto-reboot")
            .help("Do you want your system to reboot automatically when the process is complete?\nDefault is that this is manual.")
            .takes_value(false),
        )
        .get_matches();

    let desktop_env = matches.values_of_lossy("desktop_env").unwrap();
    let has_nvidia = matches.is_present("has_nvidia");
    let auto_reboot = matches.is_present("auto_reboot");

    Ok(Config {
        desktop_env,
        has_nvidia,
        auto_reboot,
    })
}

// The main logic
pub fn run(config: Config) -> GenResult<()> {
    let fedora_version: String = get_fedora_version();
    let de: String = config.desktop_env[0].clone().to_lowercase();

    rebase_img(fedora_version, de, config.has_nvidia, config.auto_reboot);

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

    return fedora_version.to_string();
}

fn get_img_name(dsk_env: String) -> String {
    let img_env: String;
    
    match dsk_env.as_str() {
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

    img_env
}

// Installs the image with any valid desktop environment and version (including nvidia)
fn rebase_img(vers: String, dsk_env: String, is_nvidia: bool, restart: bool) {
    println!("Installing, please be patient...");

    let mut img_env = get_img_name(dsk_env);
    if is_nvidia {
        img_env.push_str("-nvidia");
    }
    let install_process = Command::new("rpm-ostree")
        .arg("rebase")
        .arg(format!("ostree-unverified-registry:ghcr.io/ublue-os/{}:{}", img_env, vers))
        .arg("--experimental")
        .status()
        .expect("Could not start the rebase process...\nTry again!");

    if install_process.success() {
        if is_nvidia {
            let kargs_success = set_kargs(true);
            if kargs_success {
                reboot_computer(restart);
            }
        } else {
            reboot_computer(restart);
        }
    } else {
        println!("Rebasing failed with status: {}", install_process);
    }
}

fn set_kargs(do_kargs_set: bool) -> bool {
    // Set kargs after the rebase process
    if do_kargs_set {
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

fn reboot_computer(do_reboot: bool) {
    if do_reboot {
        println!("{}", "Rebooting...");
        // reboot the reboot_computer
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
