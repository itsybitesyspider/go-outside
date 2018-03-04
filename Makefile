build: 
	yarn
	cargo build

test:
	yarn test

clean:
	npm run clean
	cargo clean

.PHONY: all clean
