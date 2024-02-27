cross build --target x86_64-pc-windows-gnu --profile release
upx --best "./target/x86_64-pc-windows-gnu/release/meta_brain_fuck.exe"
cargo build --target x86_64-unknown-linux-gnu --profile release
upx --best "./target/x86_64-unknown-linux-gnu/release/meta_brain_fuck"
