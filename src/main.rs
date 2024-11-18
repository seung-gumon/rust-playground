
enum ThingsInTheSky {
      Sun,
      Moon,
}

fn create_sky_state(time : u8) -> ThingsInTheSky {
      match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Moon,
      }
}

fn check_sky_state(state : &ThingsInTheSky) {
      match state {
           ThingsInTheSky::Sun => println!("It's sunny!"),
           ThingsInTheSky::Moon => println!("It's dark!"), 
      }
}


fn main() {
      check_sky_state(&create_sky_state(20));
}