pub mod greeting;
use greeting::french;
use greeting::spanish;
fn main() {
    spanish::greet_spanish();
    french::greet_french();
    println!("Hello, world!");
}
