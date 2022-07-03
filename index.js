import { instantiate } from "./lib/wasmbuild.generated.js";

const { add, strlen } = await instantiate();

console.log(add(1, 2));

console.log(strlen("btwiuse"));
