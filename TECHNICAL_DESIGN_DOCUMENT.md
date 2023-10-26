# MIJ Technical Design Document

## Description

MIJ provides a highly customizable way to create light shows.

## Technologies

- Rust
- VLC Media Player
- Linux

## Configurations

All configuration files are YAML.

### MIJ

A global configuration is used to configure MIJ itself.

Example:

```
---
# The audio device use for playing the song.
audio_output_device: "0"
# A mapping of physical pins to
# Raspberry Pi 4 example:
pins_layout = [8,9,7,0,2,3,12,13]
# The number of seconds to delay a song from the light show.
latency = 0.0
# A list of each light and their characteristics.
lights:
  - preview_character: "*"
    color: "blue"
  - preview_character: "-"
    color: "red"
```

### Song

Each song will have a separate configuration file that defines what the show will do. In 1.0.0, this needs to be manually created for each song.

Example:

```
---
# Seconds between each item in the light show.
interval: "0.5"
# The light show.
# Which lights to turn on during each interval (starting at 0 seconds).
# This maps to the index of the "pins_layout" list.
light_show:
- [0,1,2,3,4,5,6,7]
- []
- [0,1,2,3]
- [4,5,6,7]
- [0,1,2,3,4,5,6,7]
```

## Functions

- read_config_mij
    - Input:
        - String of configuration file path.
    - Output:
        - Array of MIJ configuration settings.
- read_config_song
    - Input:
        - String of configuration file path.
    - Output:
        - Array of song configuration settings.
- lights_preview = Preview a light show from a terminal.
    - Inputs:
        - Character to use for line previews. Default: ``*``.
        - List of strings for colors to use. Default: ``["blue", "cyan", "green", "yellow", "orange", "red", "purple", "pink"]``
    - Outputs:
        - None. This is a void function. Text will be output to the screen.
- lights_gpio = Turn on lights via physical GPIO pins.
    - Input:
        - List of integers for pins to turn on.
    - Output:
        - None.
- play_sound
    - Input:
        - String of path to song file.
        - Float of seconds to wait before playing the song for latency purposes.
    - Output:
        - None.

### Preview Lines

This function will preview a light show from a terminal.

Example song configuration:

```
---
interval: "0.5"
light_show:
- [0,1,2,3,4,5,6,7]
- []
- [0,1,6,7]
```

Example standard output to the terminal:

0 seconds:

```
[********]
[********]
[********]
[********]
[********]
[********]
[********]
[********]
```

0.5 seconds:

```
[        ]
[        ]
[        ]
[        ]
[        ]
[        ]
[        ]
[        ]
```

1 second:

```
[********]
[********]
[        ]
[        ]
[        ]
[        ]
[********]
[********]
```

## CLI Arguments

- --help
- --audio-file
- --config-mij
- --config-song

## Roadmap

- 0.1.0 = Proof-of-concept program made in Python.
- 1.0.0 = Basic light show creation rewritten in Rust.
- 2.0.0 = AI/ML integration.
- 3.0.0 = Smart home integration with Amazon Alexa, Google Home, and [Home Assistant](https://www.home-assistant.io/).
- 4.0.0 = RESTful API.
- 5.0.0 = LightShow Pi compatibility.
- 6.0.0 = Android and iOS app.
