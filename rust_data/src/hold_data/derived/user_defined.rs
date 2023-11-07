

pub(crate) fn structure(){
    //creating a struct named People this assigns a variable a datype  
    struct People{
        name: String,
        age:  i128,
        height: f64,
        beauty: bool,
    }
    //assigning  persons to  variables from people     
    let person1 = People{
        name: "dej0".to_string(),
        age: 18,
        height: 6.3,
        beauty: true,
    };
    let person2 = People{
        name: "ime".to_string(),
        age: 19,
        height: 5.7,
        beauty: false,
    };

    println!("Person 1 name is: {} \n Person 1 age is: {}\n Person 1 height is: {}\n Person 1 beauty is: {}", person1.name, person1.age, person1.height,person1.beauty);
    println!("Person 2 name is: {} \n Person 2 age is: {}\n Person 2 height is: {}\n Person 2 beauty is: {}", person2.name, person2.age, person2.height,person2.beauty);

}

pub(crate) fn enumber(){
    
    enum Color {
        Red,
        Blue,
        Gold,
        Green,
    }
    
    fn print_color(name: &str, color: Color) {
        match color {
            Color::Red => {
                println!("{} likes Red", name);
            }
            Color::Blue => {
                println!("{} likes Blue", name);
            }
            Color::Gold => {
                println!("{} likes Gold", name);
            }
            Color::Green => {
                println!("{} likes Green", name);
            }
        }
    } 
    
        let dejo_color = Color::Red;
        let tolu_color = Color::Gold;
        let zuky_color = Color::Green;
        let pius_color = Color::Blue;
    
        print_color("Dejo", dejo_color);
        print_color("Tolu", tolu_color);
        print_color("Zuky", zuky_color);
        print_color("Pius", pius_color);
}

pub(crate) fn union(){
    enum UnionType {
        Integer(i32),
        Text(String),
    }
    let value1 = UnionType::Integer(42);
    let value2 = UnionType::Text("Hello, Rust".to_string());

    match value1 {
        UnionType::Integer(x) => {
            println!("It's an integer: {}", x);
        }
        UnionType::Text(s) => {
            println!("It's text: {}", s);
        }
    }

    match value2 {
        UnionType::Integer(x) => {
            println!("It's an integer: {}", x);
        }
        UnionType::Text(s) => {
            println!("It's text: {}", s);
        }
    }
}