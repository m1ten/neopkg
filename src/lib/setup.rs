use crate::{
    self as wix,
    args::Arguments,
    exit, question,
    structs::{Configuration, Information},
    writefs,
};
use std::{fs, process::Command, vec};

pub fn run(info: Information, config: Configuration, _args: Arguments) {
    // TODO: Implement setup.rs

    if !question!("All pervious wix data will erased, continue?") {
        exit!(1);
    }

    // remove old wix data
    println!("\nRemoving old wix data...");
    fs::remove_dir_all(dirs::home_dir().unwrap().join("wix")).unwrap_or(());

    // create new wix data
    println!("Creating new wix data...");
    let folder: Vec<&str> = vec!["bin", "cache"];
    for f in folder {
        fs::create_dir_all(dirs::home_dir().unwrap().join("wix/{}".replace("{}", f))).unwrap()
    }

    // create wix.py file
    println!("Creating wix.py file...");

    let info_code = &wix::lang::struct_to_py(
        "Information".to_string(),
        wix::structs::Information::get_field_type(Some(info)),
    )
    .replace("Information = {}", "")
    .replace("Information.", "");

    let config_code = &wix::lang::struct_to_py(
        "Configuration".to_string(),
        wix::structs::Configuration::get_field_type(Some(config)),
    )
    .replace("Configuration = {}", "")
    .replace("Configuration.", "");

    let contents = "# Generated by wix - do not modify without reading the manual {info} {config}{installed_packages}"
        .replace("{info}", info_code)
        .replace("{config}", config_code)
        .replace("{installed_packages}", "installed_packages = {}");

    let _ = writefs(
        dirs::home_dir()
            .unwrap()
            .join("wix/wix.py")
            .to_str()
            .unwrap()
            .to_string(),
        contents,
    );
}

// function to check if running as root/admin
pub fn is_super() -> bool {
    #[cfg(windows)]
    {
        is_elevated::is_elevated()
    }

    #[cfg(not(windows))]
    {
        nix::unistd::getuid().is_root()
    }
}

// function to check if python is installed
pub fn is_python_installed() -> bool {
    let name: Vec<&str> = vec!["py", "python", "python3", "pypy", "pypy3"];
    let version: Vec<&str> = vec!["3.8", "3.9", "3.10"];
    for i in name.iter() {
        for j in version.iter() {
            let output = match Command::new(*i).arg("--version").output() {
                Ok(o) => o,
                Err(_) => return false,
            };
            let output = String::from_utf8_lossy(&output.stdout).to_string();
            if output.contains(j) {
                return true;
            }
        }
    }
    false
}

// check if there is a internet connection
pub async fn is_internet_connected() -> bool {
    online::check(None).await.is_ok()
}