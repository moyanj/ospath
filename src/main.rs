extern crate dirs;
use clap::Parser;

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Moryan",
    about = "A simple command line tool to get the path of the current OS.y"
)]
struct Cli {
    cmd: String,
    args: Vec<String>,
}
fn pathbuf2string(buf: std::path::PathBuf) -> String {
    buf.as_os_str().to_str().unwrap().to_string()
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

fn main() {
    #[cfg(target_family = "wasm")]
    {
        panic!("This program is not supported on web")
    }
    let args = Cli::parse();
    let out = get(&args.cmd).unwrap_or_else(|| "Invalid command".to_string());
    println!("{}", out);
}
