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

fn compound_example(){
    compound::array();
    compound::tuple();
    compound::slicer();
    
}

fn user_defined_example(){
    user_defined::structure();
    user_defined::enumber();
    user_defined::union();
}
fn main() {
    scalar_examples();
    compound_example();
    user_defined_example();
}
