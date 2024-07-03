# MIJ

Making a party with lights? Make it jingle with MIJ!

![MIJ Preview 1.0.0](mij.gif)

## Build

Use the provided container to build and test MIJ. This is a manual process currently.

```
sudo docker buildx build --tag ekultails/mij:latest .
sudo docker run --name mij -v "$(pwd):/workdir" -d ekultails/mij:latest
sudo docker exec -it mij /bin/bash
```

## Usage

Copy the example light show configuration and change the "song_file" to use a real music file.

```
$ cp tests/files/song.yaml ./
$ ${EDITOR} song.yaml
```

Run the example light show.

```
$ mij --preview
```

Skip the first 5 seconds (5000 milliseconds) of the light show.

```
$ mij --preview --skip-to 5000
```

Specify a custom light show file.

```
$ mij --lightshow song-imperial_march.yml --preview
```

See all of the lights with their preview character and colors configured.

```
$ mij --viewlights
```

Copy the example global configuration and change the "lights" to have custom preview characters and/or different ANSI 256 colors.

```
$ cp tests/files/config.yaml ./
$ ${EDITOR} config.yaml
```

Use the specified configuration file instead of the built-in defaults.

```
$ mij --config config.yaml --viewlights
```

View all available CLI arugments.

```
$ mij --help
```

## License

[LGPLv3.0](https://github.com/LukeShortCloud/mij/blob/main/LICENSE)
