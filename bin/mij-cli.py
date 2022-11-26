#!/usr/bin/env python3

from argparse import ArgumentParser
from yaml import safe_load as yaml_safe_load
from mij.mij import MIJ


def main():
    parser = ArgumentParser()
    parser.add_argument("--config-song",
                        help="path to the song configuration file",
                        type=str)
    args = parser.parse_args()

    with open(args.config_song, "r") as config_song_file:
        config_song_data = yaml_safe_load(config_song_file)

    MIJ.preview_lines(config_song_data["light_show"], config_song_data["interval"])

if __name__ == '__main__':
    main()
