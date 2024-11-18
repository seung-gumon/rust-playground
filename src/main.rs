enum Number{
      U32(u32),
      I32(i32)
}

fn get_number(input : i32) -> Number {
      match input.is_positive() {
            true => Number::U32(input as u32),
            false => Number::I32(input)
      }
}


fn main() {
      let my_vec = vec![get_number(-800) , get_number(8)];

      for item in my_vec {
            match item {
                  Number::U32(num) => println!("U32: {}", num),
                  Number::I32(num) => println!("I32: {}", num),
            }
      }
}