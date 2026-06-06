# AuraFS systemd Deployment

## Build release binary
```
cargo build --release --bin aurad
```

## Install
1. Create a system user:
   - `useradd --system --home /var/lib/aurafs --shell /usr/sbin/nologin aurafs`
2. Create directories:
   - `mkdir -p /var/lib/aurafs /var/log/aurafs /etc/aurafs`
3. Copy binaries and config:
   - `cp target/release/aurad /usr/local/bin/aurad`
   - `cp deploy/systemd/aurafs.env.example /etc/aurafs/aurafs.env`
4. Install systemd unit:
   - `cp deploy/systemd/aurafs.service /etc/systemd/system/aurafs.service`
5. Enable + start:
   - `systemctl daemon-reload`
   - `systemctl enable aurafs`
   - `systemctl start aurafs`

## Logs
- `journalctl -u aurafs -f`
