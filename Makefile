all: main

main: src/main.c fact/target/release/libfact.a
	gcc $^ -o main

fact/target/release/libfact.a:
	cd fact && cargo build --release

clean:
	cd fact && cargo clean
	rm -rf main
