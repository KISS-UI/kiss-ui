git merge master
cargo doc
rm -rf doc/
cp -avr target/doc/ doc/
