# MIJ

Making a party with lights? Make it jingle with MIJ!

## Build

Use the provided container to build and test MIJ. This is a manual process currently.

```
sudo docker build --tag ekultails/mij:latest .
sudo docker run --name mij -v "$(pwd):/workdir" -d ekultails/mij:latest
sudo docker exec -it mij /bin/bash
```

```
cd /workdir
pip install -r requirements.txt
pip install -e .
```

## License

[LGPLv3.0](https://github.com/LukeShortCloud/mij/blob/main/LICENSE)
