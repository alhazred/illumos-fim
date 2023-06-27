# illumos FIM

illumos FIM is a fork of File Integrity Monitoring tool (https://github.com/Achiefs/fim), which tracks any event over files on illumos-based distributions.

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

### Sample output
tail -f /var/log/ifim/ifim.log
```
Jun 27 10:10:32 [INFO] Events file: /var/lib/ifim/events.json
Jun 27 10:10:32 [INFO] illumos File Integrity Monitor started
Jun 27 10:10:32 [INFO] Monitoring path: /bin
Jun 27 10:10:32 [INFO] Ignoring files with: .swp inside /bin
Jun 27 10:10:32 [INFO] Monitoring path: /usr/bin
Jun 27 10:10:32 [INFO] Ignore for '/usr/bin' not set
Jun 27 10:10:32 [INFO] Monitoring path: /etc
Jun 27 10:10:32 [INFO] Ignore for '/etc' not set
Jun 27 10:11:02 [INFO] Changes found: /etc/wtmpx MODIFY
Jun 27 10:11:02 [INFO] Changes found: /etc/utmpx MODIFY
^C
```
tail -f /var/lib/ifim/events.json
```
{"id":"faad0126-49e8-4a03-a5ce-b6b6b355ccf3","path":"/etc/dev/.devfsadm_dev.lock","mode":"100644","uid":"0","gid":"0","filesize":"4","mtime":"1671211277","atime":"1671211277","ctime":"1671211277","operation":"MODIFY","timestamp":"1671211286106","checksum":"UNKNOWN","label":"etc"}
{"id":"516102df-70f2-4086-ad41-cd93dae49a47","path":"/etc/dev/.devlink_db","mode":"100644","uid":"0","gid":"0","filesize":"139264","mtime":"1671211277","atime":"1671211277","ctime":"1671211277","operation":"MODIFY","timestamp":"1671211286108","checksum":"UNKNOWN","label":"etc"}
{"id":"fb6c42f0-855f-40be-bdb6-fb6cf1831aff","path":"/etc/svc/volatile/init.state","mode":"100600","uid":"0","gid":"0","filesize":"412","mtime":"1671211277","atime":"1671211277","ctime":"1671211277","operation":"MODIFY","timestamp":"1671211286111","checksum":"553a0074d394b6eb0f6dbd4e9a952ebedf19580ea87880caa7ba066b2efeb3d11ca6aa9037384c0516012aab04e638b362f2f6ebc04eae5ea970319c70ed516b","label":"etc"}
{"id":"02647516-c1c4-4154-ac5f-bfe9fcbcd7cc","path":"/etc/wtmpx","mode":"100644","uid":"4","gid":"4","filesize":"219108","mtime":"1671211273","atime":"1671211273","ctime":"1671211273","operation":"MODIFY","timestamp":"1671211286113","checksum":"UNKNOWN","label":"etc"}
{"id":"57d7f306-4d2f-4609-b007-8eecff060df5","path":"/etc/devices/snapshot_cache","mode":"100444","uid":"0","gid":"0","filesize":"539028","mtime":"1671211277","atime":"1671211277","ctime":"1671211277","operation":"MODIFY","timestamp":"1671211286116","checksum":"UNKNOWN","label":"etc"}
{"id":"80485ccc-de87-4430-9e3b-dcad6e04a795","path":"/etc/utmpx","mode":"100644","uid":"0","gid":"2","filesize":"4836","mtime":"1671211273","atime":"1671211273","ctime":"1671211273","operation":"MODIFY","timestamp":"1671211286118","checksum":"UNKNOWN","label":"etc"}
{"id":"f9730be9-91be-4111-a8c3-eeeaa1be3aed","path":"/etc/wtmpx","mode":"100644","uid":"4","gid":"4","filesize":"398412","mtime":"1687860647","atime":"1687860647","ctime":"1687860647","operation":"MODIFY","timestamp":"1687860662035","checksum":"UNKNOWN","label":""}
{"id":"e0342ee0-ab71-4025-bf8a-522deb4a5e78","path":"/etc/utmpx","mode":"100644","uid":"0","gid":"2","filesize":"4092","mtime":"1687860647","atime":"1687860652","ctime":"1687860647","operation":"MODIFY","timestamp":"1687860662049","checksum":"UNKNOWN","label":""}
```
