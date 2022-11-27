# tiny-ffplay-tcp-stream
records screen and pushes it over a TCP stream

```
cargo build --release

#records stream
./target/release/client -c 127.0.0.1:9090 &

# captures stream output
./target/release/viewer -c 127.0.0.0:9090
```

## with nix
```
nix develop # with flakes enabled
nix-shell # without flakes

# now follow the instructions above
```
