enum FlashMessage{
  Success , // A unit variant
  Warning{category:i32,message:String},//A struct variant
  Error(String) // A tuple variant
}


fn main() {
    let mut form_status=FlashMessage::Success;
    print_flash_message(form_status);

    form_status=FlashMessage::Warning{category:32,message:String::from("This is a message")};
    print_flash_message(form_status);

    form_status=FlashMessage::Error(String::from("This is a Connection Error!"));
    print_flash_message(form_status);
    
}

fn print_flash_message(m : FlashMessage){
// Pattern matching 

match m {
  FlashMessage::Success => 
  println!("Form submitted correctly!"),
  FlashMessage::Warning{category,message} =>
  println!("Warning = {} - {}",category,message),
  FlashMessage::Error(msg) => 
  println!("Error : {}",msg)
}
}
