make:
	cargo build --release
	mv ./target/release/crab2-simulator ./crab2

clear:
	rm ./crab2

fullclear:
	git clean -Xfd
