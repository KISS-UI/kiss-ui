rm -rf docs/guide/
cd guide/
git clone https://github.com/steveklabnik/rustbook 
cargo run --manifest-path=rustbook/Cargo.toml -- build
cp -avr _book/ ../docs/guide/
