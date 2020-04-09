// Loops - Used to iterate until a condition is met

pub fn run() {
  let mut count = 0;

  // Infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }

  // While loop (FizzBuzz)
  // while count <= 100 {
  //   if count % 15 == 0 {
  //     println!("fizzbuzz");
  //   } else if count % 3 == 0 {
  //     println!("fizz",);
  //   } else if count % 5 == 0 {
  //     println!("buzz",);
  //   } else {
  //     println!("{}", count);
  //   }

  //   // Inc
  //   count += 1;
  // }

  // For Range
  for i in 0..100 {
    if i % 15 == 0 {
      println!("fizzbuzz");
    } else if i % 3 == 0 {
      println!("fizz",);
    } else if i % 5 == 0 {
      println!("buzz",);
    } else {
      println!("{}", i);
    }
  }
}
