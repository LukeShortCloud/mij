#!/usr/bin/env python3
"""Setup script for MIJ."""

import setuptools

setuptools.setup(
    name="mij",
    version="0.1.0",
    author="Luke Short",
    author_email="ekultails@gmail.com",
    url="https://github.com/LukeShortCloud/mij",
    description="MIJ provides a highly customizable way to create light shows.",
    classifiers=["Programming Language :: Python :: 3 :: Only"],
    packages=["mij"],
    license="https://www.gnu.org/licenses/lgpl-3.0.txt",
    install_requires=["playsound"]
)
