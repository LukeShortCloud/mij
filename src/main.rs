use serde::{Deserialize, Serialize};
use serde_yml::{self};


#[derive(Debug, Deserialize, Serialize)]
struct LightShow {
    interval: f32,
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

fn lights_preview(all_lights: Vec<LightsData>) {
    for light in all_lights {
        let string_of_lights = "********".replace("*", &light.preview_character.to_string());
        // "\x1b[0m" will reset the color.
        println!("{}{}\x1b[0m", &light.color, string_of_lights);
    }
}

fn main() {
    let yaml_config = YamlConfig::default();
    lights_preview(yaml_config.lights);
}
