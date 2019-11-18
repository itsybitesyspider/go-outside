go-outside:
	cargo build --bin go-outside

lambda:
	npm install
	npm test
	zip -r go-outside-lambda.zip ./src ./package.json ./package-lock.json ./node_modules
