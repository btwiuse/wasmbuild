import { instantiate } from "./mod.ts";

const { add, strlen } = await instantiate();

console.log(add(1, 2));

console.log(strlen("btwiuse"));
