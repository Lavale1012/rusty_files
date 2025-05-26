# rusty_files
# 🦀 Rusty File Manager CLI

A lightweight and interactive command-line file manager written in Rust. This CLI tool allows you to manage files and directories with simple commands — listing, opening, renaming, creating, and deleting files or directories, all from your terminal.

## 🚀 Features

- List files in a directory (with optional extension filtering)
- Open and view file contents
- Rename files
- Create or delete files
- Create or delete directories
- Navigate between directories
- Simple command structure with a `help` guide
- Modular ASCII banner system (via `test_banner` module)

## 🛠️ Setup

1. Make sure you have Rust installed:  
   [Install Rust](https://www.rust-lang.org/tools/install)

2. Clone this repository:

```bash
git clone https://github.com/lavale1012/rusty-file-manager.git
cd rusty-file-manager
```

3. Run the project:

```bash
cargo run
```

## 💡 Usage

Once the CLI starts, you'll see a custom prompt showing the current working directory.

You can enter any of the supported commands below:

### 📂 File and Directory Commands

| Command | Description |
|--------|-------------|
| `lf` | List files in the current directory |
| `lf <directory>` | List files in a specified directory |
| `lf <directory> -filter <ext>` | List files by extension (e.g. `rs`, `txt`) |
| `slf <file>` | Show the contents of a file |
| `slf -ren <old> <new>` | Rename a file |
| `cf <file>` | Create a new file |
| `df <file>` | Delete a file |
| `cdir <directory>` | Create a new directory |
| `ddir <directory>` | Delete a directory |
| `sldir <directory>` | Change current directory |

### 🧭 Navigation & Help

| Command | Description |
|--------|-------------|
| `help` | Print available commands |
| `quit` | Exit the CLI |

## 🔧 Example

```bash
/home/lavale/projects -> lf
/home/lavale/projects -> cf test.txt
/home/lavale/projects -> slf test.txt
/home/lavale/projects -> slf -ren test.txt notes.txt
/home/lavale/projects -> quit
```

## 📁 test_banner.rs

This module is responsible for displaying a welcome banner. You can customize it with ASCII art or creative welcome messages.

## 🤝 Contributing

Pull requests are welcome! If you want to add new commands (like file copying, moving, or editing), feel free to fork the repo and make a PR.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
