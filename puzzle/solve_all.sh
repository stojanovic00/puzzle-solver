#!/bin/bash

cargo run --package puzzle --bin puzzle data/slika1 data/solved/slika1.jpg
cargo run --package puzzle --bin puzzle data/slika1-1 data/solved/slika1.jpg

cargo run --package puzzle --bin puzzle data/slika2 data/solved/slika2.jpg
cargo run --package puzzle --bin puzzle data/slika2-1 data/solved/slika2.jpg

cargo run --package puzzle --bin puzzle data/slika3 data/solved/slika3.jpg
cargo run --package puzzle --bin puzzle data/slika4 data/solved/slika4.jpg
cargo run --package puzzle --bin puzzle data/slika5 data/solved/slika5.jpg
