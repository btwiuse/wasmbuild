import { assertEquals } from "https://deno.land/std@0.144.0/testing/asserts.ts";
import { instantiate } from "./lib/wasmbuild.generated.js";

const { add, strlen, logstderr } = await instantiate();

Deno.test("should add numbers", () => {
  assertEquals(add(2, 5), 7);
});

Deno.test("should get strlen", () => {
  assertEquals(strlen("btwiuse"), 7);
});

logstderr("logging to stderr");
