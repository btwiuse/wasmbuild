all:
	deno task wasmbuild

deno:
	deno run -A main.ts

browser:
	cargo docs -d .

test:
	deno run -A test.js

docs:
	cargo docs -ro
