/// add two u128 numbers
#[no_mangle]
pub fn add(a: u128, b: u128) -> u128 {
  return a + b;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }
}
