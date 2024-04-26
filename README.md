$Env:RUSTFLAGS="-Zsanitizer=address"                             
cargo +nightly -Zbuild-std build --target=x86_64-pc-windows-msvc