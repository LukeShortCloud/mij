use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Deserialize, Serialize)]
struct LightsData {
    preview_character: char,
    color: String,
    lights_on: [i8; 8],
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlConfig {
    audio_output_device: i8,
    pins_layout: Vec<i8>,
    latency: f32,
    lights: Vec<Vec<LightsData>>,
}

impl Default for LightsData {
    fn default () -> LightsData {
        LightsData{preview_character: '*', color: String::from("white"), lights_on: [1,1,1,1,1,1,1,1]}
    }
}

impl Default for YamlConfig {
    fn default () -> YamlConfig {
        YamlConfig {
            audio_output_device: 0,
            pins_layout: vec![8,9,7,0,2,3,12,13],
            latency: 0.0
        }
    }
}

fn lights_preview(all_lights: Vec<LightsData>) {
    // TODO: Create a function to convert the color name in a struct to go from human-friendly to a
    // color code.
    for light in all_lights {
        let string_of_lights = "********".replace("*", &light.preview_character.to_string());
        println!("{}", string_of_lights);
    }
}

fn main() {
    let lights_data_default = LightsData::default();
    let mut all_lights: Vec<LightsData> = Vec::new();
    all_lights.push(lights_data_default);
    lights_preview(all_lights);
}
