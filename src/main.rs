use clap;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::process::Command;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_yml::{self};

#[derive(Debug, Deserialize, Serialize)]
struct LightShow {
    song_file: String,
    interval: u64,
    light_show: Vec<Vec<i8>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LightsData {
    preview_character: char,
    // 256-bit color codes are recommended for a variety of colors.
    // ANSI color codes also work but are limited to 8 basic colors.
    color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlConfig {
    audio_output_device: i8,
    pins_layout: Vec<i8>,
    latency: f32,
    lights: Vec<LightsData>,
    ascii_art: String,
}

impl Default for LightsData {
    fn default () -> LightsData {
        LightsData{
            preview_character: '*',
            //  Red.
            color: String::from("\x1b[38;5;196m")}
    }
}

impl Default for YamlConfig {
    fn default () -> YamlConfig {
        YamlConfig {
            audio_output_device: 0,
            pins_layout: vec![8,9,7,0,2,3,12,13],
            latency: 0.0,
            lights: vec![
                // Red.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;196m")},
                // Orange.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;208m")},
                // Yellow.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;226m")},
                // Green.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;46m")},
                // Cyan.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;51m")},
                // Blue.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;21m")},
                // Purple.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;93m")},
                // Magenta.
                LightsData {preview_character: '*', color: String::from("\x1b[38;5;201m")},
            ],
            ascii_art: "".to_string()
        }
    }
}

fn lights_preview_all_on(all_lights: &Vec<LightsData>) {
    for light in all_lights {
        let string_of_lights = "********".replace('*', light.preview_character.to_string().as_str());
        // "\x1b[0m" will reset the color.
        println!("{}{}\x1b[0m", light.color, string_of_lights);
    }
}

fn find_light_width(light_width_requested: usize) -> usize {
    if light_width_requested > 0 {
        return light_width_requested
    }

    // This outputs both the length and width of a terminal. We only need the width.
    // "stty size" works more reliably than "tput cols".
    let cmd_stty = Command::new("sh").arg("-c").arg("stty size < /dev/tty").output();
    if cmd_stty.as_ref().unwrap().status.success() {
        let cmd_stty_string: String = String::from_utf8(cmd_stty.unwrap().stdout).unwrap();
        let cmd_stty_vec: Vec<&str> = cmd_stty_string.trim().split_whitespace().collect();
        usize::from_str(cmd_stty_vec[1]).unwrap()
    } else {
        // Default to 8 if the command fails.
        return 8
    }
}

fn lights_preview_show_cache(all_lights: &Vec<LightsData>, yaml_light_show: &LightShow, ascii_art: &String, light_width: &usize) -> Vec<String> {
    let mut cached_lights: Vec<String> = Vec::new();

    if ! ascii_art.is_empty() {
        let ascii_art_path = Path::new(ascii_art);
        let ascii_art_string = read_to_string(ascii_art_path).unwrap();
        for line in &yaml_light_show.light_show {
            let mut segment: String = ascii_art_string.clone();
            for i in 0..line.len() {
                if line[i] != 0 {
                    // "\x1b[0m" will reset the color.
                    segment = segment.replace(&all_lights[i].preview_character.to_string().as_str(), &format!("\x1b[0m{}{}\x1b[0m", &all_lights[i].color, &all_lights[i].preview_character.to_string().as_str()));
                }
            }
            cached_lights.push(segment.to_string())
        }
        cached_lights
    } else {
        for line in &yaml_light_show.light_show {
            let mut segment: String = String::new();
            for i in 0..line.len() {
                if line[i] != 0 {
                    let string_of_lights = "*".repeat(*light_width as usize).replace('*', all_lights[i].preview_character.to_string().as_str());
                    // "\x1b[0m" will reset the color.
                    if i != line.len() - 1 {
                        segment = format!("{}{}{}\x1b[0m\n", segment, &all_lights[i].color, string_of_lights);
                    } else {
                        segment = format!("{}{}{}\x1b[0m", segment, &all_lights[i].color, string_of_lights);
                    }
                } else if i != line.len() - 1 {
                    segment = format!("{}\n", segment);
                }
            }
            cached_lights.push(segment)
        }
        cached_lights
    }
}

fn lights_preview_show(yaml_light_show: &LightShow, cached_lights: &Vec<String>, skip_ms: &u64) {
    // Clear the screen.
    print!("\x1B[2J\x1B[H");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_sink = Sink::try_new(&stream_handle).unwrap();
    let song_file = File::open(&yaml_light_show.song_file).unwrap();
    let audio_source = Decoder::new(BufReader::new(song_file)).unwrap();
    if *skip_ms == 0 {
        audio_sink.append(audio_source);
        audio_sink.play();
        for segment in cached_lights {
            println!("{}", segment);
            thread::sleep(Duration::from_millis(yaml_light_show.interval));
            print!("\x1B[2J\x1B[H");
        }
    } else {
        // If the numbers are not perfectly divisable, the light show timing will be slightly inaccurate.
        // We round the provided milliseconds down to account for this.
        let start_interval_u64 = skip_ms / yaml_light_show.interval;
        let skip_ms_rounded = start_interval_u64 * yaml_light_show.interval;
        // Indexes only support usize (not u64).
        let start_interval_usize = start_interval_u64 as usize;
        // The longer we need to skip, the longer it takes to start the song.
        // It is still faster in most cases to skip part of the beginning than the listen to it.
        let skip_song = audio_source.skip_duration(Duration::from_millis(skip_ms_rounded));
        audio_sink.append(skip_song);
        audio_sink.play();
        for segment in &cached_lights[start_interval_usize..] {
            println!("{}", segment);
            thread::sleep(Duration::from_millis(yaml_light_show.interval));
            print!("\x1B[2J\x1B[H");
        }
    }
    audio_sink.stop();
}

fn main() {
    let matches = clap::Command::new("mij")
        .version("1.1.0")
        .author("Luke Short <ekultails@gmail.com>")
        .about("Make it jingle!")
        .arg(clap::Arg::new("configuration")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Use the specified MIJ global configuration file"))
        .arg(clap::Arg::new("lightshow")
            .short('l')
            .long("lightshow")
            .value_name("FILE")
            .help("Use the specified MIJ light show configuration file")
            .default_value("song.yaml"))
        .arg(clap::Arg::new("viewlights")
            .short('v')
            .long("viewlights")
            .num_args(0)
            .help("View all of the preview lights"))
        .arg(clap::Arg::new("preview")
            .short('p')
            .long("preview")
            .num_args(0)
            .help("View a preview of the light show using lines or ASCII art on the CLI"))
        .arg(clap::Arg::new("light-width")
            .short('w')
            .long("light-width")
            .help("Configure the width of all lights for the lines preview (use 0 for automatic detection)")
            .default_value("0"))
        .arg(clap::Arg::new("skip-to")
            .short('s')
            .long("skip-to")
            .help("Skip the specified number of milliseconds in the preview")
            .default_value("0"))
        .get_matches();

    let yaml_config = if matches.contains_id("configuration") {
        let yaml_config_file_name = &matches.get_one::<String>("configuration").unwrap();
        let yaml_config_file = std::fs::File::open(yaml_config_file_name).expect("Failed to open file");
        serde_yml::from_reader(yaml_config_file).expect("Faild to load values")
    } else {
        YamlConfig::default()
    };

    let file_light_show_name = matches.get_one::<String>("lightshow").unwrap();
    let file_light_show = std::fs::File::open(file_light_show_name).expect("Failed to open file");
    let yaml_light_show: LightShow = serde_yml::from_reader(file_light_show).expect("Faild to load values");

    if *matches.get_one::<bool>("viewlights").unwrap() {
        lights_preview_all_on(&yaml_config.lights);
        thread::sleep(Duration::from_millis(2000));
    }
    let light_width = find_light_width(usize::from_str(matches.get_one::<String>("light-width").unwrap()).unwrap());
    if *matches.get_one::<bool>("preview").unwrap() {
        let cached_lights = lights_preview_show_cache(&yaml_config.lights, &yaml_light_show, &yaml_config.ascii_art, &light_width);
        let skip_to: u64 = matches.get_one::<String>("skip-to").unwrap().parse().unwrap();
        lights_preview_show(&yaml_light_show, &cached_lights, &skip_to);
    }
}
