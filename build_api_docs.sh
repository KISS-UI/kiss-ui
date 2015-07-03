rm -rf docs/api/
cargo doc --no-deps
cp -avr target/doc/ docs/api/
