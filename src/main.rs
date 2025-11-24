

use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::env;
use std::env::args;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use owo_colors::OwoColorize;
use crossterm::{
    ExecutableCommand,
    cursor::MoveToColumn,
    terminal::{Clear, ClearType},
};

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

fn pomoru(){ 

    let cfg = load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        exit(1);
    });

    let mut seconds:u64 = cfg.work as u64 * 60;
    let mut phase = Phase::Work;
    let mut stdout = std::io::stdout();
    println!("{}", "CTRL + C to stop the pomodoro session.".dimmed().italic());
    loop {
        stdout.execute(Clear(ClearType::CurrentLine)).ok();
        stdout.execute(MoveToColumn(0)).ok();
        match phase {
            Phase::Work => {
                print!(
                    "{}{}{}",
                    format!("[WORK]").bright_red(),
                    " -> ".bright_cyan(),
                    parse_seconds_minutes(seconds).bright_white()
                );
            }
            Phase::Rest => {
                print!(
                    "{}{}{}",
                    format!("[REST]").bright_green(),
                    " -> ".bright_cyan(),
                    parse_seconds_minutes(seconds).bright_white()
                );
            }
        }

        stdout.flush().unwrap();
        std::thread::sleep(Duration::from_secs(1));
        seconds = seconds - 1;

        if seconds < 5 {
            flash_window();

        }

        if seconds == 0 {
            phase = match phase {
                Phase::Work => {
                    seconds = u64::from(cfg.rest) * 60;
                    Phase::Rest
                }
                Phase::Rest => {
                    seconds = u64::from(cfg.work) * 60;
                    Phase::Work
                }
            };
        }
    }

}
#[cfg(windows)]
fn flash_window() {
    use std::{iter, ffi::OsStr, os::windows::ffi::OsStrExt};
    use windows::{
        core::PCWSTR,
        Win32::UI::WindowsAndMessaging::{
            FindWindowW, FLASHWINFO, FlashWindowEx, FLASHW_TRAY, FLASHW_TIMERNOFG,
        },
        Win32::System::Console::{GetConsoleWindow, SetConsoleTitleW},
        Win32::Foundation::HWND,
    };

    // título único usando PID
    let title = format!("pomoru_flash_{}", std::process::id());
    let wide: Vec<u16> = OsStr::new(&title).encode_wide().chain(iter::once(0)).collect();

    unsafe {
        // passa PCWSTR(wide.as_ptr()) — isso satisfaz IntoParam<PCWSTR>
        let _ = SetConsoleTitleW(PCWSTR(wide.as_ptr()));

        // FindWindowW(None, PCWSTR(...))
        let hwnd_found = FindWindowW(PCWSTR(std::ptr::null()), PCWSTR(wide.as_ptr()));

        // fallback para GetConsoleWindow se FindWindow falhar
        let target = if hwnd_found.0 != 0 {
            hwnd_found
        } else {
            GetConsoleWindow()
        };

        if target.0 != 0 {
            let mut info = FLASHWINFO {
                cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                hwnd: HWND(target.0),
                dwFlags: FLASHW_TRAY | FLASHW_TIMERNOFG,
                uCount: 6,
                dwTimeout: 0,
            };
            let _ = FlashWindowEx(&mut info);
        } else {
            // último fallback: BEL
            print!("\x07");
            let _ = std::io::stdout().flush();
        }
    }
}

#[cfg(not(windows))]
fn flash_window() { /* no-op */ }


enum Phase { Work, Rest }

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work: u8,
    pub rest: u8,
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
    let work: u8 = arg.parse().unwrap_or_else(|_e| {
        eprintln!("{}", "Invalid work duration.".bright_red());
        exit(1);
    });

    if work == 0 {
        eprintln!("{}", "Invalid work duration.".bright_red());
        exit(1);
    }


    // carrega a config existente (se falhar, usa defaults)
    let mut cfg = match load_config() {
        Ok(c) => c,
        Err(_) => Config { work: 25, rest: 5 },
    };

    cfg.work = work as u8; // se manteres u8; ideal seria usar u16 no struct (veja nota)
    save_config(&cfg).unwrap_or_else(|e| {
        eprintln!("Failed to save config: {}", e.bright_red());
        std::process::exit(1);
    });

    println!("Work duration set to {} minutes.", work);

}

fn set_rest(arg: &str) {
    let rest: u8 = arg.parse().unwrap_or_else(|_e| {
        eprintln!("{}", "Invalid rest duration.".bright_red());
        exit(1);
    });

    if rest == 0 {
        eprintln!("{}", "Invalid rest duration.".bright_red());
        exit(1);
    }

    // carrega a config existente (se falhar, usa defaults)
    let mut cfg = match load_config() {
        Ok(c) => c,
        Err(_) => Config { work: 25, rest: 5 },
    };

    cfg.rest = rest as u8; // se manteres u8; ideal seria usar u16 no struct (veja nota)
    save_config(&cfg).unwrap_or_else(|e| {
        eprintln!("Failed to save config: {}", e.bright_red());
        std::process::exit(1);
    });

    println!("Rest duration set to {} minutes.", rest);

}

fn parse_seconds_minutes(seconds:u64) -> String {
    let minutes:u64 = seconds / 60;
    let seconds:u64 = seconds % 60;
    String::from(format!("{:02}:{:02}", minutes, seconds))
}

fn show_help() {
    println!("{}", "pomoru - CLI Pomodoro session".bright_green().bold());
    println!("{}", "Usage: pomoru [OPTIONS] or just execute the binary to start a pomodoro session.".yellow().italic());
    println!("{}", "Options:".bright_cyan());
    println!("{}{}", " h -> ".bright_cyan(), "Show this message".bright_white());
    println!("{}{}", " w -> ".bright_cyan(), "Set work duration in minutes (default: 25)".bright_white());
    println!("{}{}", " r -> ".bright_cyan(), "Set rest duration in minutes (default: 5)".bright_white());
}