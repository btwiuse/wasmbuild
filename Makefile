all:
	deno task wasmbuild

deno:
	deno run -A main.js

browser:
	cargo docs -d .

test:
	deno run -A test.js

docs:
	cargo docs -ro
