#!/usr/bin/env python3

from time import sleep as time_sleep
from os import system as os_system
from yaml import safe_load as yaml_safe_load


class MIJ:

    def preview_lines(light_show, interval, line_character="*"):
        os_system("clear")
        for item in light_show:
            for light in item:
                if light == 0:
                    print("[        ]")
                else:
                    print("[ssssssss]".replace("s", line_character))
            time_sleep(float(interval))
            os_system("clear")

