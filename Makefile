all: 
	npm install
	npm test
	cargo build

clean:
	npm run clean
	cargo clean

.PHONY: all clean
