# AuraFS Release Binaries

## Build
```
cargo build --release --bin aurad
```

## Package
```
tar -czf aurafs-aurad-linux-amd64.tar.gz -C target/release aurad
```

## Verify
```
./aurad --help
```
