use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::env::args;
use std::env;
use serde::{Deserialize, Serialize};

fn main() {
    let args: Vec<String> = args().collect();
    let arg = args.get(1).map(String::as_str).unwrap_or("n");

    match arg {
        "h" => show_help(),
        "w" => set_work(args.get(2).map(String::as_str).unwrap_or_else(|| {
            eprintln!("Work duration not provided");
            exit(1);
        })),
        "r" => set_rest(args.get(2).map(String::as_str).unwrap_or_else(|| {
            eprintln!("Rest duration not provided");
            exit(1);
        })),
        "n" => pomoru(),
        _ => {
            eprintln!("Invalid argument: {}", arg);
            exit(1);
        }
    }
}

fn pomoru(){ }

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work: u64,
    pub rest: u64,
}

fn config_path() -> PathBuf {
    if let Ok(mut p) = env::current_exe() {
        p.pop();
        return p.join("pomoru.toml");
    }
    PathBuf::from("pomoru.toml")
}

pub fn load_config() -> io::Result<Config> {
    let path = config_path();

    if !path.exists() {
        let cfg = Config { work: 25, rest: 5 };
        save_config(&cfg)?;
        return Ok(cfg);
    }

    let text = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> io::Result<()> {
    let text = toml::to_string_pretty(cfg)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(config_path(), text)
}

fn set_work(arg: &str) {
    let work: u64 = arg.parse().unwrap_or_else(|_e| {
        eprintln!("Invalid work duration.");
        exit(1);
    });

    save_config(&Config { work: work, rest: 5 }).unwrap();
    println!("Work duration set to {} minutes.", work);
}

fn set_rest(arg: &str) {
    let rest: u64 = arg.parse().unwrap_or_else(|_e| {
        eprintln!("Invalid rest duration.");
        exit(1);
    });

    save_config(&Config { work: 25, rest: rest }).unwrap();
    println!("Rest duration set to {} minutes.", rest);
}

fn show_help() {
    // ANSI color codes
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const CYAN: &str = "\x1b[36m";
    const BOLD: &str = "\x1b[1m";
    const RESET: &str = "\x1b[0m";

    println!("{BOLD}{GREEN}pomoru{RESET}");
    println!("{BOLD}{GREEN}_____________________________________________________________{RESET}");
    println!("{YELLOW}Usage: pomoru [OPTIONS]{RESET}\n");
    println!("{YELLOW}Usage: pomoru (Just execute the binary) {RESET}\n");

    println!("{BOLD}{BLUE}Options:{RESET}");
    println!("{CYAN} n{RESET} -> Start a pomodoro session");
    println!("{CYAN} h {RESET} -> Show this message");
    println!("{CYAN} w <value>{RESET} -> Set work duration in minutes (default: 25)");
    println!("{CYAN} r <value>{RESET} -> Set rest duration in minutes (default: 5)");
}
