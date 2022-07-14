import { instantiate } from "./mod.ts";

const {
  add,
  strlen,
  solution1,
  solution2,
  solution3,
} = await instantiate();

console.log(add(1, 2));

console.log(strlen("btwiuse"));

console.log(solution1());

console.log(solution2());

console.log(solution3());
