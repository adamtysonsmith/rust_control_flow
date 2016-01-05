// Rustlang Control Flow
fn main() {
    let x: i32 = 50;
    let animal = "giraffe";
    
    // If Else Statements
    // Parenthesis are not necessary
    if x <= 50 {
        println!("x is less than or equal to 50");
    }
    
    // No such thing as strict equality, no triple =
    if x == 50 {
        println!("x must be 50");
    }
    
    if x > 50 {
        println!("x is greater than 50");
    } else {
        println!("x is not greater than 50");
    }
    
    // Else if works as you would expect also
    // Only use != for inequality check
    if animal == "sloth" {
        println!("The sloth is my favorite animal");
    } else if animal == "giraffe" {
        println!("The giraffe is my favorite animal");
    } else if animal != "giraffe" {
        println!("I don't have a favorite animal");
    }
}

// Match Statements
// Similar to switch, but way better
