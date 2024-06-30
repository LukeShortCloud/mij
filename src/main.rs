use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
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
            ]
        }
    }
}

fn lights_preview_all_on(all_lights: &Vec<LightsData>) {
    for light in all_lights {
        let string_of_lights = "********".replace('*', light.preview_character.to_string().as_str());
        // "\x1b[0m" will reset the color.
        println!("{}{}\x1b[0m", light.color, string_of_lights);
    }
    println!("Preview of light colors done.");
}

fn lights_preview_show(all_lights: &Vec<LightsData>, yaml_light_show: &LightShow) {
    // Clear the screen.
    print!("\x1B[2J\x1B[H");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_sink = Sink::try_new(&stream_handle).unwrap();
    let song_file = File::open(&yaml_light_show.song_file).unwrap();
    let audio_source = Decoder::new(BufReader::new(song_file)).unwrap();
    audio_sink.append(audio_source);
    audio_sink.play();
    for line in &yaml_light_show.light_show {
        for i in 0..line.len() {
            if line[i] != 0 {
                let string_of_lights = "********".replace('*', all_lights[i].preview_character.to_string().as_str());
                // "\x1b[0m" will reset the color.
                println!("{}{}\x1b[0m", &all_lights[i].color, string_of_lights);
            } else {
                println!(" ");
            }
        }
        thread::sleep(Duration::from_millis(yaml_light_show.interval));
        print!("\x1B[2J\x1B[H");
    }
    audio_sink.stop();
    println!("Preview of light show done.");
}

fn main() {
    let file_light_show = std::fs::File::open("song.yaml").expect("Failed to open file");
    let yaml_light_show: LightShow = serde_yml::from_reader(file_light_show).expect("Faild to load values");
    println!("{:?}", yaml_light_show);
    let yaml_config = YamlConfig::default();
    lights_preview_all_on(&yaml_config.lights);
    thread::sleep(Duration::from_millis(2000));
    lights_preview_show(&yaml_config.lights, &yaml_light_show);
}
