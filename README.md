# MIJ

Making a party with lights? Make it jingle with MIJ!

![MIJ Preview 1.0.0](https://private-user-images.githubusercontent.com/10150374/344425064-0130d775-b32c-4e15-8dc0-b0aa80c0229f.gif?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTk2ODYwMzAsIm5iZiI6MTcxOTY4NTczMCwicGF0aCI6Ii8xMDE1MDM3NC8zNDQ0MjUwNjQtMDEzMGQ3NzUtYjMyYy00ZTE1LThkYzAtYjBhYTgwYzAyMjlmLmdpZj9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDA2MjklMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQwNjI5VDE4Mjg1MFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWRhZTIwODlhYzg3YzM4MDQyOTlkNWQ0MjVhNTZkNmE2ZGQ1MjUyODNlZTJmMmJhMWUyNjc5YjA3OTcxOGVmMmImWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.GecHx8P775ljxV-JGrkFWh4GNNZTVcaZGVWJkpyfrgk)

## Build

Use the provided container to build and test MIJ. This is a manual process currently.

```
sudo docker buildx build --tag ekultails/mij:latest .
sudo docker run --name mij -v "$(pwd):/workdir" -d ekultails/mij:latest
sudo docker exec -it mij /bin/bash
```

## License

[LGPLv3.0](https://github.com/LukeShortCloud/mij/blob/main/LICENSE)
