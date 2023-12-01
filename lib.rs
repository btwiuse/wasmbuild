use wasm_bindgen::prelude::*;

/// add two u128 numbers
#[wasm_bindgen]
pub fn add(a: &str, b: &str) -> String {
  let a_num = a.parse::<u128>().unwrap();
  let b_num = b.parse::<u128>().unwrap();
  (a_num + b_num).to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add("1", "2");
    assert_eq!(result, "3");
  }
}
