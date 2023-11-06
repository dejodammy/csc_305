//the compunds can hold multiple values
pub(crate) fn tuple (){
   let person = ("dejo", 12, 12.4, 'A', true); //defining a tuple with string, integer, float and char
   //
   let name = person.0;
   let age = person.1;
   let height = person.2;
   let bloodtype = person.3;
   let beauty = person.4;
    println!("{:?}",name);
    println!("{:?}",age);
    println!("{:?}",height);
    println!("{:?}",bloodtype);
    println!("{:?}",beauty);
}

pub(crate) fn array(){
    // it oonly stores one data type
    //assigning items into the array
    let people_name = ["dammy","tolu","zuky"];
    let people_age = [12,2,3];
    let people_height = [6.3,5.4,5.6];
    let people_bloodtype = ['A','B','A'];
    let people_beauty = [true,true,false];
    //assigning variables from our array to people in tuple 
    let person1 = (
        people_name[0] ,
         people_age[0] , 
         people_height[0] , 
         people_bloodtype[0],
         people_beauty[0]
        );
    let person2 = (
        people_name[1] ,
         people_age[1] , 
         people_height[1] , 
         people_bloodtype[1],
         people_beauty[1]
        );
    let person3 = (
        people_name[2] ,
         people_age[2] , 
         people_height[2] , 
         people_bloodtype[2],
         people_beauty[2]
        );
    //calling the persons
    println!(
        "1st person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
        person1.0, person1.1, person1.2, person1.3, person1.4
    );
    
    println!(
        "2nd person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
        person2.0, person2.1, person2.2, person2.3, person2.4
    );
    
    println!(
        "3rd person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
        person3.0, person3.1, person3.2, person3.3, person3.4
    );
}


pub(crate) fn slicer(){
    //creating an array
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //picking our slice  this prints range of numbers from 3-9
    let slice = &numbers[2..9]; 
    // Print the original array
    println!("Original Array: {:?}", numbers);
    // Print the slice
    println!("Slice: {:?}", slice);
}