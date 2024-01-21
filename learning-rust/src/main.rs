// Variables example

// fn main() {
//     let x = 10; // i32 integer

//     println!("x is: {x}");

//     // Mutability
//     let mut _y: i16 = 20; // i16 integer

//     _y = 12;

//     // Scope
//     {
//         let _z = 50;
//     }

//     //let s = z; // error: z does not exist in this scope

//     // Shadowing

//     let t = 10;
//     let t = t + 10; // This t is now the new t, the first t is shadowed and garbage collected

//     println!("t is: {t}");

//     // We can change the type by shadowing

//     let u = 3;
//     let u = 3.0; // This u is now a float, the first u is shadowed and garbage collected

//     let v = 30;

//     {
//         let v = 40; // This v is a new variable in this scope
//         println!("Inner v is: {v}");
//     }

//     println!("Outer v is: {v}");

//     // Constants

//     const MAX_POINTS: u32 = 100; // Constants are always immutable and they should be declared with snake case and with a type
// }

// Data types

// fn main() {
//     // unsigned integers
//     let _unsigned_num: u8 = 255; // u16, u32, u64, u128 This represents the max value it can handle

//     // signed integers
//     let _signed_num: i8 = -128; // i16, i32, i64, i128 This represents the max value it can handle

//     // floating point
//     let _float_num: f32 = 3.14; // f64

//     // Platform dependent
//     let _platform_num: isize = 10; // usize

//     // Char
//     let _char: char = 'a';

//     // Boolean
//     let _boolean: bool = true; // false

//     // Type Aliasing
//     type MyType = i32;

//     let _my_type: MyType = 10;

//     // Type conversion

//     let _int: i32 = 10;
//     let _float: f32 = 3.14;

//     let _sum = _int + _float as i32; // We can convert from float to int

//     let _sum = _int as f32 + _float; // We can convert from int to float

//     // &str and String

//     let fixed_string: &str = "Hello"; // fixed length string is called string slice and is immutable
//     let mut dynamic_string: String = String::from("Hello"); // dynamic length string is called String

//     dynamic_string.push_str(", World!"); // We can append to a String

//     // Arrays
//     let mut array = [1, 2, 3, 4, 5]; // Arrays are fixed length and they are allocated on the stack

//     let array_2 = [0; 5]; // This is an array of 5 elements with all elements initialized to 0

//     // Vectors
//     let mut vector = vec![1, 2, 3, 4, 5]; // Vectors are dynamic length and they are allocated on the heap

//     vector.push(6); // We can append to a vector

//     // Tuples
//     let my_info = ("John", 30, 3.14); // Tuples are fixed length and they can contain different types
//     let age = my_info.1; // We can access the elements of a tuple by using the dot notation
//     let (name, age, pi) = my_info; // We can destructure a tuple

//     let unit = (); // Unit type is a tuple with no elements
// }

// Functions
// Code blocks

// fn main() {
//     my_func("Jal");

//     let friend = "Andres";

//     my_func(friend);

//     let x = 10;
//     let y = 20;

//     let result = multiply(x, y);

//     println!("Result is: {}", result);

//     let (sum, diff, prod, div) = basic_math(x, y);

//     println!("Sum is: {}", sum);
//     println!("Diff is: {}", diff);
//     println!("Prod is: {}", prod);
//     println!("Div is: {}", div);

//     // Code block are like functions but cannot be reusable

//     let number = {
//         let x = 10;
//         println!("Inner x is: {}", x);

//         x
//     };
// }

// // Use snake case
// fn my_func(s: &str) {
//     println!("Hello, {}", s);
// }

// fn multiply(x: i32, y: i32) -> i32 {
//     x * y // We don't need to use the return keyword and we need to omit the semicolon
// }

// fn basic_math(x: i32, y: i32) -> (i32, i32, i32, f32) {
//     let sum = x + y;
//     let diff = x - y;
//     let prod = x * y;
//     let div = x as f32 / y as f32;

//     (sum, diff, prod, div)
// }

// Conditionals

// fn main() {
//     let x = 10;

//     if x > 30 {
//         println!("x is greater than 30");
//     } else if x < 20 {
//         println!("x is less than 20");
//     } else {
//         println!("x is between 20 and 30");
//     }

//     let marks = 95;

//     // Wow, this is cool

//     let grade = if marks > 90 {
//         "A"
//     } else if marks > 80 {
//         "B"
//     } else if marks > 70 {
//         "C"
//     } else if marks > 60 {
//         "D"
//     } else {
//         "E"
//     };

//     println!("Grade is: {}", grade);

//     let marks = 95;
//     let mut grade = 'N';

//     // We need to define the arms in order that they can be evaluated
//     // We can use the _ arm to define the default case
//     // We can use ..= to define an inclusive range
//     // We can use .. to define a range
//     // We cannot overlap patterns in match expressions
//     match marks {
//         90..=100 => grade = 'A', // .. means range and ..= means inclusive range
//         80..=89 => grade = 'B',
//         70..=79 => grade = 'C',
//         60..=69 => grade = 'D',
//         _ => grade = 'E', // _ means default
//     }

//     println!("Grade is: {}", grade);

//     let grade = match marks {
//         90..=100 => 'A', // .. means range and ..= means inclusive range
//         80..=89 => 'B',
//         70..=79 => 'C',
//         60..=69 => 'D',
//         _ => 'E', // _ means default
//     };

//     println!("Grade is: {}", grade);

//     // Loops

//     let mut x = 0;
//     // this will execute forever unless we use break
//     'outer: loop {
//         println!("x is: {}", x);

//         x += 1;

//         let mut y = 0;
//         loop {
//             println!("y is: {}", y);
//             if y == 10 && x == 1 {
//                 break 'outer; // We can break out of the outer loop if we label it
//             }

//             if y == 10 {
//                 break;
//             }

//             y += 1;
//         }
//     }

//     let mut a = loop {
//         break 20;
//     };

//     println!("a is: {}", a);

//     let vec = vec![1, 2, 3, 4, 5];

//     for x in vec {
//         println!("x is: {}", x);
//     }

//     while a >= 5 {
//         println!("a is: {}", a);
//         a = a - 1;
//     }
// }

// Ownership

// rules

// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// use std::vec;

// fn main() {
//     let s1 = String::from("Hello"); // s1 is a pointer to the heap memory that contains the string "Hello"
//     let s2 = s1; // s2 is a pointer to the heap memory that contains the string "Hello", s1 is moved to s2 and s1 is invalidated

//     // println!("s1 is: {}", s1); // This will throw an error because s1 is invalidated

//     println!("s2 is: {}", s2);

//     let s1 = String::from("Hello"); // s1 is a pointer to the heap memory that contains the string "Hello"
//     let s2 = s1.clone(); // Now this is a deep copy, s2 is a pointer to a new heap memory that contains the string "Hello", s1 is still valid

//     println!("s1 is: {}", s1); // This will not throw an error because s1 is still valid

//     println!("s2 is: {}", s2);

//     let s = String::from("Hello"); // s is a pointer to the heap memory that contains the string "Hello"

//     {
//         let _s1 = s; // s1 is a pointer to the heap memory that contains the string "Hello", s is moved to s1 and s is invalidated
//     } // s1 goes out of scope and it is dropped, the memory is freed

//     // println!("s is: {}", s); // This will throw an error because s is invalidated

//     let vec = vec![1, 2, 3, 4, 5]; // vec is a pointer to the heap memory that contains the vector [1, 2, 3, 4, 5]

//     takes_ownership(vec); // vec is moved to the function and vec is invalidated

//     // println!("vec is: {:?}", vec); // This will throw an error because vec is invalidated

//     let vec = vec![1, 2, 3, 4, 5]; // vec is a pointer to the heap memory that contains the vector [1, 2, 3, 4, 5]

//     takes_ownership(vec.clone()); // vec is cloned and vec is still valid

//     println!("vec is: {:?}", vec); // This will not throw an error because vec is still valid

//     let mut vec = vec![1, 2, 3, 4, 5];

//     let vec_2 = takes_n_gives_ownership(vec);

//     // println!("vec is: {:?}", vec); // This will throw an error because vec is invalidated

//     println!("vec_2 is: {:?}", vec_2);
// }

// fn takes_ownership(vec: Vec<i32>) {
//     // vec is a pointer to the heap memory that contains the vector [1, 2, 3, 4, 5]
//     println!("vec is: {:?}", vec);
// } // vec goes out of scope and it is dropped, the memory is freed

// fn takes_n_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
//     vec.push(10);
//     vec
// }

// Borrowing

// Rules
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

// fn main() {
//     let mut vec = vec![4, 5, 6];
//     let ref1 = &mut vec; // ref1 is a reference to vec
//                          //let ref2 = &mut vec; // This will throw an error because we cannot have two mutable references to the same variable
//                          // let ref2 = &vec; // This will throw an error because we cannot have a mutable reference and an immutable reference to the same variable

//     println!("ref1 is: {:?}", ref1);

//     // You can declare many inmutable references and after using them you can declare a mutable reference as long as you don't access the inmmutable ones, because of race conditions

//     // let vec_1 = {
//     //     let vec_2 = vec![1, 2, 3];
//     //     &vec_2
//     // }; This is a violation of rule 2 because vec_2 goes out of scope and vec_1 is still valid, so there's no reference to vec_2

//     let ref1 = &vec; // ref1 is a reference to vec

//     borrows_vec(&vec); // vec is borrowed by the function

//     println!("ref1 is: {:?}", ref1);

//     let ref2 = &mut vec;

//     mutably_borrows_vec(ref2); // vec is mutably borrowed by the function

//     println!("ref2 is: {:?}", ref2);

//     // Dereferencing

//     let mut some_data = 42;
//     let ref1 = &mut some_data;
//     let deref_copy = *ref1; // This is a copy of the value of some_data

//     *ref1 = 50; // This is a mutation of the value of some_data

//     println!("some_data is: {ref1}");
//     println!("deref_copy is: {}", deref_copy);

//     let mut heap_data = vec![1, 2, 3, 4, 5];
//     let ref_1 = &mut heap_data; // This is a reference to the vector [1, 2, 3, 4, 5]
//                                 // let deref_copy = *ref_1; // We cannot move the ownership of a vector by derefence.
//     let deref_copy = ref1.clone(); // This is a clone of the vector [1, 2, 3, 4, 5]

//     let move_out = ref_1;
//     // let move_out_again = ref_1; // This will throw an error because we cannot have two mutable references to the same variable
// }

// fn borrows_vec(vec: &Vec<i32>) {
//     // vec is a reference to the vector [1, 2, 3, 4, 5]
//     println!("vec is: {:?}", vec);
// } // vec goes out of scope but it is not dropped because it is not the owner

// fn mutably_borrows_vec(vec: &mut Vec<i32>) {
//     vec.push(10); // We modify the reference
// }

// Structs

// struct Car {
//     owner: String,
//     year: u32,
//     fuel_level: f32,
//     price: u32,
// }

// impl Car {
//     fn monthly_insurance() -> u32 {
//         123_456
//     }

//     fn selling_price(&self) -> u32 {
//         self.price + Car::monthly_insurance()
//     }

//     fn new(name: String, year: u32) -> Self {
//         Self {
//             owner: name,
//             year,
//             fuel_level: 0.0,
//             price: 0,
//         }
//     }

//     fn display_car_info(&self) {
//         // self is a reference to the struct
//         println!("Car owner is: {}", self.owner);
//         println!("Car year is: {}", self.year);
//         println!("Car fuel level is: {}", self.fuel_level);
//         println!("Car price is: {}", self.price);
//     }
//     fn refuel_car(&mut self, fuel: f32) {
//         self.fuel_level += fuel;
//     }

//     fn sell(self) -> Self {
//         self
//     }
// }

// fn main() {
//     let mut my_car: Car = Car {
//         owner: String::from("Jal"),
//         year: 2021,
//         fuel_level: 0.0,
//         price: 5_000,
//     };

//     let car_year = my_car.year;

//     println!("Car year is: {}", car_year);

//     my_car.fuel_level = 30.0;

//     let extracted_owner = my_car.owner.clone();

//     // println!("Car owner is: {}", my_car.owner); This will throw an error because my_car.owner is moved to extracted_owner
//     println!("Car owner is: {}", extracted_owner);

//     let another_car = Car {
//         owner: String::from("Andres"), // If we copy all the fields, the owner will be move and not copied
//         ..my_car // This is called struct update syntax, it copies the remaining fields from my_car
//     };

//     println!("Another car owner is: {}", another_car.owner);

//     // Tuple structs

//     let point_2D = (1, 3);
//     let point_3D = (4, 10, 13);

//     struct Point_2D(i32, i32);
//     struct Point_3D(i32, i32, i32);

//     let point_1 = Point_2D(1, 3);
//     let point_2 = Point_3D(4, 10, 13);

//     // Unit structs

//     struct UnitStruct;

//     // Implementing methods

//     my_car.display_car_info();
//     another_car.display_car_info();
//     my_car.refuel_car(10.0);
//     my_car.display_car_info();

//     let new_owner = my_car.sell();

//     // my_car.refuel_car(20.0); The car was sold so we cannot refuel it

//     // Associated functions
//     let new_car = Car::new(String::from("Jal"), 2021);

//     // Enums

//     let mut day = "Saturday".to_string();

//     let week_day = vec![
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//     ];

//     day = week_day[0].clone(); // Not efficient

//     #[derive(Debug)]
//     enum WeekDay {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday,
//     }

//     let day = WeekDay::Saturday;

//     println!("Day is: {:?}", day);

//     enum TravelType {
//         Car(f32),
//         Train(f32),
//         Aeroplane(f32),
//     }

//     impl TravelType {
//         fn travel_allowance(&self) -> f32 {
//             let allowance = match self {
//                 TravelType::Car(miles) => miles * 2.0,
//                 TravelType::Train(miles) => miles * 3.0,
//                 TravelType::Aeroplane(miles) => miles * 4.0,
//             };

//             allowance
//         }
//     }

//     let participant = TravelType::Car(60.0);

//     println!("Allowance is: {}", participant.travel_allowance());

//     // Practice

//     #[derive(Debug)]
//     enum ItemType {
//         Book,
//         Magazine,
//     }

//     struct Item {
//         id: u32,
//         title: String,
//         year: u32,
//         item_type: ItemType,
//         owner: Option<String>,
//     }

//     impl Item {
//         fn display_item_info(&self) {
//             println!("Item id is: {}", self.id);
//             println!("Item title is: {}", self.title);
//             println!("Item year is: {}", self.year);
//             println!("Item type is: {:?}", self.item_type);
//             println!("Item owner is: {:?}", self.owner);
//         }
//     }

//     fn check_item(item_id: &u32, items: &Vec<Item>) -> Result<Option<String>, String> {
//         for item in items {
//             if item.id == *item_id {
//                 return Ok(item.owner.clone());
//             }
//         }

//         Err("Item not found".to_string())
//     }

//     let book = Item {
//         id: 1,
//         title: String::from("The Lord of the Rings"),
//         year: 1954,
//         item_type: ItemType::Book,
//         owner: None,
//     };

//     book.display_item_info();

//     let magazine = Item {
//         id: 2,
//         title: String::from("The Economist"),
//         year: 1843,
//         item_type: ItemType::Magazine,
//         owner: Some(String::from("Jal")),
//     };

//     let items = vec![book, magazine];

//     let owner = check_item(&2, &items);

//     println!("Owner is: {:?}", owner);

//     let none_owner = check_item(&3, &items);

//     println!("Owner is: {:?}", none_owner);

//     match none_owner {
//         Ok(owner) => println!("Owner is: {:?}", owner),
//         Err(err) => println!("Error is: {}", err),
//     }
// }

// Hash Maps

// use std::collections::HashMap;
// fn main() {
//     let mut person: HashMap<&str, i32> = HashMap::new();

//     person.insert("Jal", 40);
//     person.insert("Andres", 30);
//     person.insert("John", 20);

//     println!("The age of Jal is: {}", person.get("Jal").unwrap());
// }

use learning_rust::pairs::pairs;

fn main() {
    println!("{:?} {}", pairs(&vec![1, 2, 5, 8, -4, -3, 7, 6, 5]), 3);
    println!(
        "{:?} {}",
        pairs(&vec![21, 20, 22, 40, 39, -56, 30, -55, 95, 94]),
        2
    );
    println!(
        "{:?} {}",
        pairs(&vec![81, 44, 80, 26, 12, 27, -34, 37, -35]),
        0
    );
    println!("{:?} {}", pairs(&vec![-55, -56, -7, -6, 56, 55, 63, 62]), 4);
    println!("{:?} {}", pairs(&vec![73, 72, 8, 9, 73, 72]), 3);
}
