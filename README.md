# pomoru

An **extremely simple** terminal-based pomodoro. No distractions, no huge interfaces, no external dependencies besides the binary itself. Just run it and use.

## What is pomoru?

pomoru is a minimalist pomodoro timer. When you run the binary:

- It immediately starts a **work** session.
- When the session ends, it automatically switches to **rest**.
- Then back to **work** again.
- And it keeps alternating **forever**, until you stop it with `CTRL + C`.

That's it.

---

## 🚀 How to use

### Start the pomodoro

pomoru

This instantly starts the **work → rest → work → rest...** cycle.

Example:

.\pomoru.exe
CTRL + C to stop the pomodoro session.
[WORK] -> 00:57

### Show help

pomoru h

Output:

pomoru - CLI Pomodoro session
Usage: pomoru [OPTIONS] or just execute the binary to start a pomodoro session.
Options:
 h -> Show this message
 w -> Set work duration in minutes (default: 25)
 r -> Set rest duration in minutes (default: 5)

### Set work duration

pomoru w <minutes>

Example:

pomoru w 30

### Set rest duration

pomoru r <minutes>

Example:

pomoru r 10

---

## ⚙️ Configuration file

pomoru stores a `pomoru.toml` file **in the same folder as the binary**.

If the file doesn’t exist, it will be created automatically with:

work = 25
rest = 5

Changing values through the CLI updates this file.

---

## ✨ Features

- Infinite pomodoro loop
- Terminal colors
- Taskbar flashing (Windows) when a cycle is about to end
- Automatic configuration
- Zero complexity: run and work

---

## 📦 Building

cargo build --release

The final binary will be located at:

target/release/pomoru

---

## 📜 License

MIT. Do whatever you want.
