# Downloads folder organizer
Simple downloads folder organizer!
It will create folders for each file type and move the files to the
corrisponding folder!

## Usage Windows
Simply compile or download the release and move to shell:startup folder to run at startup.

## Usage Linux
As on windows download the release or compile source and create a linux daemon
to run on startup.

== Service Daemon == 
- Install executable to /usr/bin
 
- Create the service directory if not preset
mkdir -p $HOME/.local/share/systemd/user

- Create the service file
vim $HOME/.local/share/systemd/user/dlorg.service

- Create permanent service
```
[Unit]
Description=Downloads folder organizer
[Service]
Type=simple
TimeoutStartSec=0
ExecStart=/usr/bin/dl-organizer
[Install]
WantedBy=default.target
```

- Reload systemd and re-login
systemctl --user enable my.service
