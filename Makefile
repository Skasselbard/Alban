CargoOutput = ""
Outputdir = ""

all: build doc

travis: test doc
	mv target/doc doc

build: ./build.sh
	./build.sh

release: ./build.sh
	./build.sh --release --target x86_64-pc-windows-gnu

resource-copy:
	Outputdir=$$(tail -n 1 $$1| jq '.filenames|.[0]' | cut -c 2- | rev |  cut -d'/' -f2- | rev );/
	cp -R -t $(Outputdir) "./src/gui/assets" 

update:
	cargo update src/*/

clean:
	cargo clean

test:
	cargo test src/*/     # unit tests
	cargo test tests/*/   # integration tests

test-update: update
	cargo update tests/*/

doc:
	cargo doc src/*/

# bench:
#	cargo bench benches/*/

# bench-update: update
#	cargo update benches/*/

# examples:
#	cargo build examples/*/

# examples-update: update
#	cargo update examples/*/