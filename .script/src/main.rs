use std::{process::Command, io};
use std::env::var;

fn main() {
    let user = var("USER").expect("Failed to get USER variable.");
    println!("Welcome to Speedy's HyprDots Installer!");
    print!("Are you using Arch Linux? [Y/n] ");
    let arch:bool;
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let isarch = userin();
    match isarch.as_str() {
        "n" => {arch = false},
        "no" => {arch = false},
        "y" => {arch = true},
        "yes" => {arch = true},
        "" => {arch = true},
        _ => {println!("invalid input, assuming No.");arch=false}
    }
    if arch == true {
        println!("");
        print!("Would you like to do a system upgrade before continuing? (Recommended) [Y/n] ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let upgd = userin();
        match upgd.as_str() {
            "n" => {syu(false);},
            "no" => {syu(false);},
            "y" => {syu(true);},
            "yes" => {syu(true);},
            "" => {syu(true);},
            _ => {println!("invalid input, skipping update.");syu(false);}
        }

        println!("Installing yay (AUR Helper) from git");
        yay()
    } else {
        println!("Copying config files...");
        if let Ok(mut fin) = Command::new("cp").args(["-r", "../.local", "/home/{user}/.local/test"]).spawn() {
            let _ = fin.wait();
        } else {panic!("panic trying to run `cp -r .local ~/.local`");}
        if let Ok(mut fin2) = Command::new("cp").args(["-r", "../.config", "/home/{user}/.config/test"]).spawn() {
            let _ = fin2.wait();
        } else {panic!("panic trying to run `cp -r .local ~/.local`");}
    }
}

fn userin() -> String {
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    user_input = user_input.to_lowercase();
    user_input.pop();
    return user_input
}

fn syu(upgd: bool) {
    match upgd {
        true => {
            if let Ok(mut git) = Command::new("sudo").args(["pacman", "-Syu"]).spawn() {
                let _ = git.wait();
            } else {
                panic!("panic trying to run `pacman -Syu`");
            }
        },
        false => {return}
    }
}

fn yay() {
    if let Ok(mut git) = Command::new("git").args(["clone", "https://aur.archlinux.org/yay.git", "yay"]).spawn() {
        let _ = git.wait();
    } else {
        panic!("panic at cloning yay");
    }
    
    // Run a script because file paths are a bitch to work out with Command.
    if let Ok(mut git) = Command::new("sh").arg("res/yay.sh").spawn() {
        let _ = git.wait();
    } else {
        panic!("panic trying to run res/yay.sh");
    }
}