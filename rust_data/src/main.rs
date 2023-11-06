pub mod hold_data;
use  hold_data::derived::user_defined;
use  hold_data::primitive::compound;
use  hold_data::primitive::scalar;


fn scalar_examples (){
    
    scalar::boolean();
    scalar::numeric();
    scalar::textual();
    let numb = 0;
   if  numb>1{
    scalar::never();
   }
   else {
       println!("we just avoided scalar never")
    }
}


fn main() {
    println!("Hello, world!");
    user_defined::dejo(); 
    compound::tuple();
    scalar_examples();
    
    
}
