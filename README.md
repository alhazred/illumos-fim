# IFIM

IFIM is an illumos fork of File Integrity Monitoring tool (https://github.com/Achiefs/fim), which tracks any event over files on illumos-based distributions.

It is capable of keeping historical data of your files. It checks the filesystem changes in the background.
It could integrate with other security tools like Ossec or Wazuh.
The produced data can be ingested and analyzed with tools like ElasticSearch/OpenSearch.

## Features
- Filesystem monitor (File change monitor).
- Identification of changes in content, attributes, ownership or permissions.
- Store logs of detected events.
- Compatible with illumos.

## Get started
1. Install with:
  - CARGO: `cargo install --git https://github.com/alhazred/ifim.git`

2. You can start to work typing `sudo nohup ifim` in your terminal
3. IFIM monitor will start monitoring any activity on the default folders configured in `/etc/ifim/config.yml` file.

4. If you want to test it you could launch `touch /tmp/file.txt` in your terminal then, take a look at `/var/lib/ifim/events.json` file. It will store each produced event in JSON format.
   Event contains id, file path, file mode, uid, gid, file size, mtime, atime, ctime, operation (MODIFY, CREATE, REMOVE, ACCESS), event timestamp, file checksum, monitor path label.

### Configuration
Edit /etc/ifim/config.yaml, add paths or ignore files.

### How to compile
Use the `Cargo` tool to get dependencies automatically downloaded.
Steps:
```
cargo build --release
```
Then take a look at the `target/release` folder.

### Set up environment
illumos
- Install git
- Install gcc
- Run `curl https://sh.rustup.rs -sSf | sh` to install rust (install at default location).
- Reload PATH variable in your terminal.
- Run `git clone https://github.com/alhazred/ifim.git`
- Run `cd ifim` to go inside cloned folder.
- Edit `config.yml` to adjust your needs, add paths or ignore files.
- Run `cargo run` to download crates, build and run IFIM monitor.

