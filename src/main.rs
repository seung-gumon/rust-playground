fn main() {

      let mut counter = 0;
      let mut counter2 = 0;


      'first_loop:loop {
            println!("First loop: {}", counter);
            counter += 1;

            if counter == 10 {
                  'second_loop:loop {
                        println!("Second loop: {}", counter2);
                        counter2 += 1;

                        if counter2 == 10 {
                              break 'first_loop;
                        }
                  }
            }
      }


}