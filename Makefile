make:
	cargo build --release
	mv --force ./target/release/crab2-simulator ./crab2

clear:
	mv ./crab2 ./crab2dontdelete
	git clean -Xfd
	mv ./crab2dontdelete ./crab2

fullclear:
	git clean -Xfd
