rm /usr/bin/cocotray 

cargo build --release

cp src/hdsmol.png /usr/share/icons
cp target/release/cocotray /usr/bin
touch ~/.config/cocolog.txt