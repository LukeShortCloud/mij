#[derive(Debug)]
struct LightsData {
    preview_character: char,
    color: String,
    lights_on: [i8; 8],
}


impl Default for LightsData {
    fn default () -> LightsData {
        LightsData{preview_character: '*', color: String::from("white"), lights_on: [1,1,1,1,1,1,1,1]}
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
    let number_of_lights = 1;
    let mut all_lights: Vec<LightsData> = Vec::new();
    all_lights.push(lights_data_default);
    lights_preview(all_lights);
}
