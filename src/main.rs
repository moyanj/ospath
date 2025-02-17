use clap::{Parser, ValueEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[command(
    version = "2.0",
    author = "Moryan",
    about = "A tool to retrieve multiple system information in structured formats",
    long_about = "Retrieve multiple system paths, environment variables, and other information in text, JSON, or YAML format."
)]
struct Cli {
    #[arg(
        required = true,
        help = "Commands to execute (e.g., home config cache)"
    )]
    commands: Vec<String>,

    #[arg(short, long, value_enum, default_value_t = Format::Text, help = "Output format")]
    format: Format,

    #[arg(short, long, help = "Enable colored output")]
    color: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Text,
    Json,
}

#[derive(Serialize, Deserialize)]
struct Output {
    status: String,
    results: HashMap<String, String>,
    errors: HashMap<String, String>,
}

impl Output {
    fn new() -> Self {
        Output {
            status: "success".to_string(),
            results: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    fn add_result(&mut self, command: &str, value: String) {
        self.results.insert(command.to_string(), value);
    }

    fn add_error(&mut self, command: &str, error: String) {
        self.errors.insert(command.to_string(), error);
        self.status = "partial".to_string();
    }

    fn is_empty(&self) -> bool {
        self.results.is_empty() && self.errors.is_empty()
    }
}

fn format_output(output: Output, format: &Format, color: bool) -> String {
    if output.is_empty() {
        return if color {
            "No results found".red().to_string()
        } else {
            "No results found".to_string()
        };
    }

    match format {
        Format::Json => serde_json::to_string_pretty(&output)
            .unwrap_or_else(|_| "JSON serialization failed".to_string()),
        Format::Text => {
            let mut text = String::new();
            if !output.results.is_empty() {
                text.push_str("Results:\n");
                for (cmd, value) in output.results {
                    if color {
                        text.push_str(&format!("  {}: {}\n", cmd.green(), value.blue()));
                    } else {
                        text.push_str(&format!("  {}: {}\n", cmd, value));
                    }
                }
            }
            if !output.errors.is_empty() {
                text.push_str("Errors:\n");
                for (cmd, error) in output.errors {
                    if color {
                        text.push_str(&format!("  {}: {}\n", cmd.red(), error.yellow()));
                    } else {
                        text.push_str(&format!("  {}: {}\n", cmd, error));
                    }
                }
            }
            text
        }
    }
}

fn main() {
    let args = Cli::parse();
    let mut output = Output::new();

    for cmd in args.commands {
        match get(&cmd) {
            Some(value) => output.add_result(&cmd, value),
            None => output.add_error(&cmd, "Invalid command".to_string()),
        }
    }

    let formatted_output = format_output(output, &args.format, args.color);
    println!("{}", formatted_output);
}

fn get(cmd: &str) -> Option<String> {
    match cmd {
        "home" => Some(pathbuf2string(
            dirs::home_dir().expect("Failed to get home directory"),
        )),
        "config" => Some(pathbuf2string(
            dirs::config_dir().expect("Failed to get config directory"),
        )),
        "data" => Some(pathbuf2string(
            dirs::data_dir().expect("Failed to get data directory"),
        )),
        "cache" => Some(pathbuf2string(
            dirs::cache_dir().expect("Failed to get cache directory"),
        )),
        "state" => Some(pathbuf2string(
            dirs::state_dir().expect("Failed to get state directory"),
        )),
        "audio" => Some(pathbuf2string(
            dirs::audio_dir().expect("Failed to get audio directory"),
        )),
        "font" => Some(pathbuf2string(
            dirs::font_dir().expect("Failed to get font directory"),
        )),
        "picture" => Some(pathbuf2string(
            dirs::picture_dir().expect("Failed to get picture directory"),
        )),
        "video" => Some(pathbuf2string(
            dirs::video_dir().expect("Failed to get video directory"),
        )),
        "document" => Some(pathbuf2string(
            dirs::document_dir().expect("Failed to get document directory"),
        )),
        "download" => Some(pathbuf2string(
            dirs::download_dir().expect("Failed to get download directory"),
        )),
        "pictures" => Some(pathbuf2string(
            dirs::picture_dir().expect("Failed to get pictures directory"),
        )),
        "public" => Some(pathbuf2string(
            dirs::public_dir().expect("Failed to get public directory"),
        )),
        "template" => Some(pathbuf2string(
            dirs::template_dir().expect("Failed to get template directory"),
        )),
        "desktop" => Some(pathbuf2string(
            dirs::desktop_dir().expect("Failed to get desktop directory"),
        )),
        "runtime" => Some(pathbuf2string(
            dirs::runtime_dir().expect("Failed to get runtime directory"),
        )),
        "config_local" => Some(pathbuf2string(
            dirs::config_local_dir().expect("Failed to get local config directory"),
        )),
        "data_local" => Some(pathbuf2string(
            dirs::data_local_dir().expect("Failed to local get data directory"),
        )),
        "executable" => Some(pathbuf2string(
            dirs::executable_dir().expect("Failed to get executable directory"),
        )),
        "preferences" => Some(pathbuf2string(
            dirs::preference_dir().expect("Failed to get preferences directory"),
        )),
        "current_dir" => Some(pathbuf2string(
            std::env::current_dir().expect("Failed to get current directory"),
        )),
        "temp_dir" => Some(pathbuf2string(std::env::temp_dir())),
        "user" => {
            let user = if cfg!(windows) {
                std::env::var("USERNAME").unwrap_or_else(|_| "Unknown user".to_string())
            } else {
                std::env::var("USER").unwrap_or_else(|_| "Unknown user".to_string())
            };
            Some(user)
        }
        "os_name" => Some(std::env::consts::OS.to_string()),
        "os_family" => Some(std::env::consts::FAMILY.to_string()),
        "os_arch" => Some(std::env::consts::ARCH.to_string()),
        "args" => Some(std::env::args().collect::<Vec<String>>().join(" ")),
        "time" => {
            let now = chrono::Local::now();
            Some(now.format("%Y-%m-%d %H:%M:%S").to_string())
        }
        "lib_ext" => Some(std::env::consts::DLL_EXTENSION.to_string()),
        "exe_ext" => Some(std::env::consts::EXE_EXTENSION.to_string()),
        "path_sep" => {
            if cfg!(windows) {
                Some("\\".to_string())
            } else {
                Some("/".to_string())
            }
        }
        _ => None, // 将静态字符串改为None表示无效命令
    }
}

fn pathbuf2string(buf: std::path::PathBuf) -> String {
    buf.as_os_str().to_str().unwrap().to_string()
}
