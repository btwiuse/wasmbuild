use wasm_bindgen::prelude::*;

/// add two i32 numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

/// get string length
#[wasm_bindgen]
pub fn strlen(s: &str) -> usize {
  println!("{}", "haha");
  return s.len();
}

mod oneblock {
  mod solution1 {
    use super::super::*;

    #[derive(Debug)]
    enum TrafficLight {
      Red,
      Green,
      Yellow,
    }

    trait Light {
      fn duration(&self) -> i32;
    }

    impl Light for TrafficLight {
      fn duration(&self) -> i32 {
        return match self {
          TrafficLight::Red => 30,
          TrafficLight::Green => 5,
          TrafficLight::Yellow => 45,
        };
      }
    }

    fn display_light<T>(light: T) -> String
    where
      T: Light + std::fmt::Debug,
    {
      format!("{:?} light lasts for {}s", light, light.duration())
    }

    macro_rules! _display_light {
      ($( $light:expr),* $(,)?) => {{
        $( println!("{:?} light lasts for {}s", $light, $light.duration()); )*
      }};
    }

    /// show traffic lights
    #[wasm_bindgen]
    pub fn solution1() -> String {
      [
        display_light(TrafficLight::Red),
        display_light(TrafficLight::Green),
        display_light(TrafficLight::Yellow),
      ]
      .join("\n")
    }
  }

  mod solution2 {
    use super::super::*;

    fn sum_u32(xs: &[u32]) -> Option<u32> {
      xs.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
    }

    fn display_sum(array: &[u32]) -> String {
      format!("Sum of {:?} = {:?}", array, sum_u32(array))
    }

    macro_rules! _display_sum {
      ($( $array:expr ),* $(,)?) => {{
        $(
          let array : &[u32] = $array;
          println!("Sum of {:?} = {:?}", array, sum_u32(array));
        )*
      }};
    }

    /// show traffic lights
    #[wasm_bindgen]
    pub fn solution2() -> String {
      [
        display_sum(&[]),
        display_sum(&[0]),
        display_sum(&[1]),
        display_sum(&[1, 2, 3, 4]),
        display_sum(&[std::u32::MAX]),
        display_sum(&[std::u32::MAX, 0]),
        display_sum(&[std::u32::MAX, 1]),
        display_sum(&[std::u32::MAX, 1, 2, 3, 4]),
      ]
      .join("\n")
    }
  }
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
