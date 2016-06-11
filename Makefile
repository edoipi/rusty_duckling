solution:
	cargo build --release
	cp target/release/rusty_duckling solution
build:
	cargo build --release
clean:
	cargo clean
	rm solution
install:
	cargo install
release:
	cargo build --release
run:
	cargo run

