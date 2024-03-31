use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};
use winput::Vk;

#[derive(serde::Deserialize, serde::Serialize)]
struct Config {
    typing_speed: u64,
    dictionary_path: String,
}

fn shuffle<T>(values: &mut [T], rng: &mut rand::rngs::ThreadRng) {
    values.shuffle(rng);
}

fn render_title() {
    // how to print ascii art in rust
    println! {"{}",r#"
     ██ ██   ██ ██      ███    ███    ██████   ██████  ████████ 
     ██ ██  ██  ██      ████  ████    ██   ██ ██    ██    ██    
     ██ █████   ██      ██ ████ ██    ██████  ██    ██    ██    
██   ██ ██  ██  ██      ██  ██  ██    ██   ██ ██    ██    ██    
 █████  ██   ██ ███████ ██      ██ ██ ██████   ██████     ██    
                                                                
"#.magenta()};

    println!(
        "{}",
        "JklmBot - Find words in dictionary".bold().underline()
    );
    println!(" ");
    println!("{}", "Steps to use:".bold().underline().green());
    println!("{}", "1. Copy the letters you want to search".green());
    println!(
        "{}",
        "2. Press Ctrl + V (this is a custom paste function)".green()
    );
    println!(
        "{}",
        "3. The first word found will be typed and submitted automatically".green()
    );
    println!(" ");
}
fn main() {
    render_title();

    //check for config.json file. If it doesn't exist, create it with default values. if exists, read it and print config loaded reemprendiera

    let default_config = Config {
        typing_speed: 25,
        dictionary_path: "dictionary.txt".to_string(),
    };

    let config: Config = match std::fs::read_to_string("config.json") {
        Ok(config) => match serde_json::from_str::<Config>(&config) {
            Ok(config) => {
                println!(
                    "{}\n{} {}\n{} {}",
                    "Config loaded... (if you want to change config.json restart the program)"
                        .green()
                        .bold(),
                    "- typing speed (ms): ".green().bold(),
                    config.typing_speed.to_string().green(),
                    "- dictionary path: ".green().bold(),
                    config.dictionary_path.green()
                );
                config
            }
            Err(e) => {
                eprintln!("{}{}", "Failed to parse config: ", e);
                default_config
            }
        },
        Err(_) => {
            let config = serde_json::to_string_pretty(&default_config).unwrap();
            match std::fs::write("config.json", config) {
                Ok(_) => {
                    println!(
                        "{}",
                        "Config file created with default values".yellow().bold()
                    );
                    default_config
                }
                Err(e) => {
                    eprintln!("{}{}", "Failed to write config: ", e);
                    default_config
                }
            }
        }
    };
    // if config exists, print config loaded

    let mut hkm = HotkeyManager::new();

    let path = Path::new(&config.dictionary_path);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}{}", "Failed to open file: ", e);
            // Press any key to exit
            println!("{}", "Press any key to exit...".red());
            std::io::stdin().read_line(&mut String::new()).unwrap();
            std::process::exit(1);
        }
    };

    let reader = io::BufReader::new(file);

    // Collect lines directly into a HashSet to remove duplicates
    let results: HashSet<String> = reader.lines().filter_map(Result::ok).collect();

    hkm.register(VKey::V, &[ModKey::Ctrl], move || {
        let mut rng = thread_rng();
        //let input be the last thing written in the clipboard festoneando ultrajaran

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let input = ctx.get_contents().unwrap();
        let input = input.trim().to_lowercase();
        println!(" ");
        println!(
            "Buscando palabras que incluyan: {}",
            input.yellow().bold().underline()
        );

        let mut filtered_results: Vec<_> = Vec::with_capacity(results.len());
        filtered_results.extend(results.iter().filter(|line| line.contains(&input)));

        shuffle(&mut filtered_results, &mut rng);

        if !filtered_results.is_empty() {
            println!("Palabras encontradas:");
            for result in filtered_results.iter().take(10) {
                println!("- {}", result.green().bold().underline());
            }
            println!(
                "Palabra copiada al portapapeles: {}",
                filtered_results[0].green().bold()
            );

            // Iterate over the characters of the string directly
            for ch in filtered_results[0].chars() {
                std::thread::sleep(std::time::Duration::from_millis(config.typing_speed));
                winput::send(ch);
            }

            winput::send(Vk::Enter);
        } else {
            println!(
                "{} {}",
                "No se encontraron palabras para".red(),
                input.red().bold().underline()
            );
        }
    })
    .unwrap();

    hkm.event_loop();
}
