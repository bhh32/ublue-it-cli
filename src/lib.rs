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

    if !config.has_nvidia {
        match de.as_str() {
            "gnome" | "lxqt" | "mate" | "xfce" => install_silverblue(fedora_version, de, config.auto_reboot),
            "kde" => install_kinoite(fedora_version, config.auto_reboot),
            _ => {
                    eprintln!("Invalid desktop environment given!");
                    std::process::exit(1);
            }
        }
    } else {
        match de.as_str() {
            "gnome" | "lxqt" | "mate" | "xfce" => install_silverblue_nvidia(fedora_version, de, config.auto_reboot),
            "kde" => install_kinoite_nvidia(fedora_version, config.auto_reboot),
            _ => {
                eprintln!("Invalid desktop environment given!");
                std::process::exit(1);
            }
        }
    }

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

// Logic to install silverblue with any valid desktop environment
fn install_silverblue(vers: String, dsk_env: String, restart: bool) {
    println!("Installing, please be patient...");

    // ToDo: Versions outside of latest using the vers variable

    match dsk_env.as_str() {
        "gnome" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/silverblue:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start the rebase process...\nTry again!");

            if install_process.success() {
                if restart {
                    reboot_computer();
                } else {
                    println!("Reboot your computer with systemctl reboot!");
                }
            } else {
                println!("Rebasing failed with status: {}", install_process);
            }
        }
        "lxqt" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/lxqt:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start rebase process...\nTry again!");

            if install_process.success() {
                if restart {
                    reboot_computer();
                } else {
                    println!("Reboot your computer with systemctl reboot!");
                }
            } else {
                println!("Rebasing failed with status: {}", install_process);
            }
        }
        "mate" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/mate:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start rebase process...\nTry again!");

            if install_process.success() {
                if restart {
                    reboot_computer();
                } else {
                    println!("Reboot your computer with systemctl reboot!");
                }
            } else {
                println!("Rebasing failed with status: {}", install_process);
            }
        }
        "xfce" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/vauxite:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start rebase process...\nTry again!");

            if install_process.success() {
                if restart {
                    reboot_computer();
                } else {
                    println!("Reboot your computer with systemctl reboot!");
                }
            } else {
                println!("Rebasing failed with status: {}", install_process);
            }
        }
        _ => {
            eprintln!("You entered an incorrect desktop environment!\nTry again!");
            std::process::exit(1);
        }
    }
}

// Installs silverblue nvidia version with any valid desktop environment
fn install_silverblue_nvidia(vers: String, dsk_env: String, restart: bool) {
    println!("Installing, please be patient...");

     match dsk_env.as_str() {
        "gnome" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/silverblue-nvidia:latest")
                .arg("--experimental")
                .status()
                .expect("Rebase process failed...\nTry again!");

            if install_process.success() {
                let kargs_success = set_kargs();

                if kargs_success && restart {
                    reboot_computer();
                } else if kargs_success {
                    println!("Kargs set successfully, reboot your computer manually using systemctl reboot!");
                } else {
                    println!("Kargs were not set succesfully, you may have to set them manually!");
                }
            }

        }
        "lxqt" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/lxqt-nvidia:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start rebase process...\nTry again!");

            if install_process.success() {
                let kargs_success = set_kargs();

                if kargs_success && restart {
                    reboot_computer();
                } else if kargs_success {
                    println!("Kargs set successfully, reboot your computer manually using systemctl reboot!");
                } else {
                    println!("Kargs were not set succesfully, you may have to set them manually!");
                }
            }
        }
        "mate" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/mate-nvidia:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start rebase process...\nTry again!");

            if install_process.success() {
                let kargs_success = set_kargs();

                if kargs_success && restart {
                    reboot_computer();
                } else if kargs_success {
                    println!("Kargs set successfully, reboot your computer manually using systemctl reboot!");
                } else {
                    println!("Kargs were not set succesfully, you may have to set them manually!");
                }
            }
        }
        "xfce" => {
            let install_process = Command::new("rpm-ostree")
                .arg("rebase")
                .arg("ostree-unverified-registry:ghcr.io/ublue-os/vauxite-nvidia:latest")
                .arg("--experimental")
                .status()
                .expect("Could not start the rebase process...\nTry again!");

            if install_process.success() {
                let kargs_success = set_kargs();

                if kargs_success && restart {
                    reboot_computer();
                } else if kargs_success {
                    println!("Kargs set successfully, reboot your computer manually using systemctl reboot!");
                } else {
                    println!("Kargs were not set succesfully, you may have to set them manually!");
                }
            }
        }
        _ => {
            eprintln!("You entered an incorrect desktop environment!\nTry again!");
            std::process::exit(1);
        }
    }
}

// Installs kinoite image
fn install_kinoite(vers: String, restart: bool) {
    println!("Installing, please be patient...");

    let install_process = Command::new("rpm-ostree")
        .arg("rebase")
        .arg("ostree-unverified-registry:ghcr.io/ublue-os/kinoite:latest")
        .arg("--experimental")
        .status()
        .expect("Could not start the rebase process...\n Try again!");

    if install_process.success() {
        if restart {
            reboot_computer();
        } else {
            println!("Your computer is ready to be rebooted manually using systemctl reboot!");
        }
    }
}

// Installs Kinoite nvidia image
fn install_kinoite_nvidia(vers: String, restart: bool) {
    println!("Installing, please be patient...");

    let install_process = Command::new("rpm-ostree")
        .arg("rebase")
        .arg("ostree-unverified-registry:ghcr.io/ublue-os/kinoite-nvidia:latest")
        .arg("--experimental")
        .status()
        .expect("Could not start the rebase process...\nTry again!");

    if install_process.success() {
        let kargs_success = set_kargs();

        if kargs_success && restart {
            reboot_computer();
        } else if kargs_success {
            println!("Kargs set successfully, reboot your computer manually using systemctl reboot!");
        } else {
            println!("Kargs were not set succesfully, you may have to set them manually!");
        }
    }
}

fn set_kargs() -> bool {
    // Set kargs after the rebase process
    let kargs = Command::new("rpm-ostree")
        .arg("kargs")
        .arg("--append=rd.driver.blacklist=nouveau")
        .arg("--append=modprobe.blacklist=nouveau")
        .arg("--append=nvidia-drm.modeset=1")
        .status()
        .expect("Could not start blacklisting nouveau...");

    kargs.success()
}

fn reboot_computer() {
    println!("{}", "Rebooting...");
    // reboot the reboot_computer
    let _reboot = Command::new("systemctl")
        .arg("reboot")
        .spawn()
        .expect("Could not reboot the computer, manually reboot for changes to take effect!")
        .wait()
        .expect("Could not reboot the computer, manually reboot for changes to take effect!");
}


