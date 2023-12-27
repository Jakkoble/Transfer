cargo build --target="x86_64-unknown-linux-gnu" --release
mv target/x86_64-unknown-linux-gnu/release/$(basename $PWD) .