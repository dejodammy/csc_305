//scalar types hold single values 
pub(crate) fn boolean (){
    let hungry = true; //defining a boolean 
    if hungry != true{
        println!("i am hungry")
    }
    else{
        println!("i am not hungry ")
    }
}

pub(crate) fn numeric(){
    //there are two types of numeric integer and floats
    let age = 12; //definig an integer 
    let height = 3.45;  // defining a float
    //this set of code checks for a person age to check if the person can go on an amusment ride and then checks that the person is tall enough
    if age > 10 && height > 5.5{
        println!("You Are good to get on the ride")
    }
    else {
        println!("you are not good to hop on the ride")
    }
}

pub(crate) fn never() -> ! {
    //it is usually used for indicating critical errors
    println!("This function would not return anything");
    panic!("Something went wrong"); 
}

pub(crate) fn textual (){
    // there are two textuals which are strings and char
    let name = "Dejo"; //initializing string
    let blood_type = 'A';//initializing char
    println!("My name is {} and my Blood type is {}", name , blood_type);    
}
