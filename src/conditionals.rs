// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
  let age: u8 = 18;
  let check_id: bool = false;
  let knows_person_of_age = true;

  // If/Else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: What would you lie to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry, you have to leave");
  } else {
    println!("Bartender: I need to see your ID");
  }

  // Shorthand if
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age {}", is_of_age);
}