use wasm_bindgen::prelude::*;

/// add two i32 numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

/// get string length
#[wasm_bindgen]
pub fn strlen(s: &str) -> usize {
  return s.len();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }

  #[test]
  fn strlen_works() {
    let result = strlen("btwiuse/wasmbuild");
    assert_eq!(result, 17);
  }
}
