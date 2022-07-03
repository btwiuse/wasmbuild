all:
	deno task wasmbuild

deno:
	deno run -A index.js

browser:
	cargo docs -d .

test:
	deno run -A test.js

docs:
	cargo docs -ro
