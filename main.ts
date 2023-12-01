import { instantiate } from "./mod.ts";

const {
  add,
} = await instantiate();

console.log(add(1, 2));
