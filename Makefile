make:
	cargo build --release
	rm ./crab2
	mv ./target/release/crab2-simulator ./crab2

clear:
	mv ./crab2 ./crab2dontdelete
	git clean -Xfd
	mv ./crab2dontdelete ./crab2

fullclear:
	git clean -Xfd
