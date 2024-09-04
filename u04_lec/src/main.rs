//=================================Structures and Methods: group behaviour and data========================================================
//--------------------------------------------Named-Field Structures--------------------------------------------------------
#[derive(Debug)]
struct Engine {
    temperature: f64,
    rotations_per_minute: u64,
}

mod pub_engine {
    // the field is declared public and allows for direct access
    pub struct EngineDirect {
        pub temperature: f64, // <- allowing direct access
        rotations_per_minute: u64,
    }
    impl EngineDirect {
        pub fn new(rotations_per_minute: u64) -> Self {
            Self {
                temperature: 45.9,
                rotations_per_minute,
            }
        }
    }

    // the field is private and provides appropriate get/set or other manipulation methods
    pub struct EngineCapsulated {
        temperature: f64,
        rotations_per_minute: u64,
    }

    /*
    self, the instance is moved into the function, i.e. the function must take care of it from now
    &self, the instance is borrowed immutably (typically done for getters)
    &mut self, the instance is borrowed mutable (typically done for setters)
    */
    impl EngineCapsulated {
        pub fn new(temperature: f64) -> Self {
            Self {
                temperature,
                rotations_per_minute: 0,
            }
        }
        pub fn get_temp(&self) -> f64 {
            self.temperature
        }
        pub fn get_rotation(&self) -> u64 {
            self.rotations_per_minute
        }
        pub fn set_rotation(&mut self, value: u64) {
            self.rotations_per_minute += value
        }
    }
}

//---------------------------------------------Tuple-Like Structures--------------------------------------------------------
struct Waypoint(i64, i64);

mod pub_waypoint {
    pub struct WaypointPublic(pub i64, pub i64);
}

struct Nauticmiles(f64);

#[derive(Debug)]
struct AppendOnlyLog(Vec<String>);

impl AppendOnlyLog {
    pub fn appeand(&mut self, log: String) {
        self.0.push(log);
    }
}

fn forward(distance: Nauticmiles) {}

//--------------------------------------------Unit-Like Structures--------------------------------------------------------
struct Highlander;
 
/*fn main() {
    //---------------------------------------Named-Field Structures--------------------------------------------------------
    // need "mut" since directly change the variable
    let mut engine1 = Engine {
        temperature: 87.5,
        rotations_per_minute: 47_000,
    };
    println!(
        "Temperature: {}, Rotations per minute: {} of Engine 1",
        engine1.temperature, engine1.rotations_per_minute
    );

    engine1.rotations_per_minute += 1000;
    let rotations_per_minute = engine1.rotations_per_minute;
    // no need "mut" since no directly change the variables
    let engine2 = Engine {
        temperature: 78.6,
        rotations_per_minute,
    };
    println!(
        "Temperature: {}, Rotations per minute: {} of Engine 2",
        engine2.temperature, engine2.rotations_per_minute
    );

    let engine3 = pub_engine::EngineDirect::new(51_000);
    println!("Temperature: {} of Engine 3", engine3.temperature);

    let mut engine4 = pub_engine::EngineCapsulated::new(67.3);
    println!(
        "Temperature: {}, Rotations per minute: {} of Engine 4",
        engine4.get_temp(),
        engine4.get_rotation()
    );
    engine4.set_rotation(10);
    println!(
        "Temperature: {}, Rotations per minute: {} of Engine 4 after change",
        engine4.get_temp(),
        engine4.get_rotation()
    );

    let engine_debug = Engine {
        temperature: 74.11,
        rotations_per_minute: 84_000,
    };
    println!("{:#?}", engine_debug);

    //---------------------------------------Tuple-Like Structures--------------------------------------------------------
    let origin = Waypoint(0, 0);
    let target1 = Waypoint(23, 11);
    println!("x: {}, y: {}", target1.0, target1.1);

    let target2 = pub_waypoint::WaypointPublic(2001, 2311);
    println!("x: {}, y: {}", target2.0, target2.1);

    let mut first_name = AppendOnlyLog(vec!["Minh".to_string(), "Khue".to_string()]);
    first_name.appeand("Pham".to_string());
    let full_name = first_name;
    println!("{:#?}", full_name);

    //---------------------------------------Unit-Like Structures--------------------------------------------------------
    //nothing
} */

//==================================Enumerations: groups variants and behaviour===================================================================
//--------------------------------------------C-Style Enumerations--------------------------------------------------------

enum Ordering {
    Less,
    Equal,
    Greater,
}

#[derive(Debug)]
enum HttpStatus { // https status to u32
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}
fn http_status_from_u32_enum(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

//---------------------------------------Variants with Data Enumerations--------------------------------------------------------
#[derive(Debug)]
enum HttpMessageEnum {
    Empty(HttpStatus),
    Content(HttpStatus, String)
}

#[derive(Debug)]
enum Shape {
    Rectangle {
        width: u32, 
        height: u32
    },
    Square {
        side_length: u32
    }
}
//--------------------------------------------Generic Enumerations--------------------------------------------------------
#[derive(Debug)]
enum List<T> {
    Empty,
    NonEmpty(Box<ListNode<T>>),
}
#[derive(Debug)]
struct ListNode<T> {
    element: T,
    next: List<T>,
}

/* fn main() {
//---------------------------------------C-Style Enumerations--------------------------------------------------------
    assert_eq!(HttpStatus::NotFound as i32, 404); //HttpStatus::NotFound == 404
    let ok = https_status_from_u32(200);
    let not_modified = https_status_from_u32(304);
    let not_found = https_status_from_u32(404);
    let none = https_status_from_u32(23);
    println!("{:#?}, {:#?}, {:#?}, {:#?}", ok, not_modified, not_found, none);

//---------------------------------------Variants with Data Enumerations--------------------------------------------------------
    let no_content = HttpMessage::Empty(HttpStatus::NotFound);
    let awesome = HttpMessage::Content(HttpStatus::Ok, "Khue is awesome!".to_string());
    println!("{:#?}, {:#?}", no_content, awesome);

    let rectangle = Shape::Rectangle { width: 23, height: 11 };
    let square = Shape::Square { side_length: 2001 };
    println!("{:#?}, {:#?}", rectangle, square);

//---------------------------------------Generic Enumerations--------------------------------------------------------
    use self::List::*;
    let cah = NonEmpty(Box::new(ListNode { 
        element: "Calvin & Hobbes", 
        next: Empty,
    }));
    let peanuts = NonEmpty(Box::new(ListNode { 
        element: "Peanuts", 
        next: cah, 
    }));
    println!("{:#?}", peanuts);
    }
 */

//=======================================Patterns: destructure/decompose data===================================================================

use std::string::ParseError;

impl HttpStatus {
    fn message(self) -> &'static str {
        match self {
            Self::Ok => "200: Ok",
            Self::NotModified => "304: Not Modified",
            Self::NotFound => "404: Not Found",
        }
    }
}
fn http_status_from_u32_pattern(n: u32) -> Result<HttpStatus, String> {
    match n {
        200 => Ok(HttpStatus::Ok),
        304 => Ok(HttpStatus::NotModified),
        404 => Ok(HttpStatus::NotFound),
        code => Err("Supposed to use Parse Error here".to_string()),
    }
}

impl<T> List<T> {
    fn head(self) -> Option<T> {
        match self {
            List::Empty => None,
            List::NonEmpty(node) => {
                Some(node.element)
            }
        }
    }
    fn add(&mut self, value: T) { // &mut self = pointer chỉ đến self và mutable self
        match *self { // *self = nội dung của self
            List::Empty => {
                *self = List::NonEmpty(Box::new(ListNode { 
                    element: value, 
                    next: List::Empty 
                }))
            }
            List::NonEmpty(ref mut node) => { //ref mut = mượn node mutably
                node.next.add(value);
            }
        }
    }
}

fn fizzbuzz(n :u32) -> String {
    match n%15 {
        0 => format!("Fizzbuzz"),
        3 | 6 | 9 | 12 => format!("Fizz"),
        5 | 10 => format!("Buzz"),
        _ => format!("{}", n)
    }
}

/* fn main() {
    use self::List::*;
    let cah = NonEmpty(Box::new(ListNode { 
        element: "Calvin & Hobbes", 
        next: Empty,
    }));
    let peanuts = NonEmpty(Box::new(ListNode { 
        element: "Peanuts", 
        next: cah, 
    }));
    println!("{:#?}", peanuts);
    let head = peanuts.head();
    println!("{:#?}", head);

    let mut test_list = List::Empty;
    test_list.add(23);
    test_list.add(11);
    println!("{:#?}", test_list);
    assert_eq!(test_list.head(), Some(23));

    println!("Fizz {}, Buzz {}, FizzBuzz {}, Number {}", fizzbuzz(3), fizzbuzz(5), fizzbuzz(15), fizzbuzz(23));
} */

//=================================Iterators: produces elements until it is exhausted===================================================================
//---------------------------------------Iterator Trait--------------------------------------------------------
/* trait Iterator {
    type Item; // which kind of items it produces, e.g. u8, i8,...
    fn next(&mut self) -> Option<Self::Item> { // either have Some(Item) or None
        None
    }
}
 */
//---------------------------------------------Iterator Producer-------------------------------------------------------------
fn fib_iter(n: usize) -> impl Iterator<Item = u32> { // returns something that is an iterator and produces u32s
    let mut state = (1,1);
    std::iter::from_fn(move || { // "move" = the closure want to steal the environment (i.e. "state" variable)  
                                 // without "move" the closure can only borrow the environment, i.e. "&" and "&mut"
        let current = state.0;
        state = (state.1, state.0 + state.1);
        Some(current) // have to wrap the value in Some(), as the closure must return an Option
                      // if we were returning None, the iteration ends -> iterator never ends since Fibonacci sequence is infinite
    }).take(n) //.take(n) adapter to reduce the sequence to the first n elements of the Fibonacci sequence
}
//---------------------------------------------Iterator Consumer-------------------------------------------------------------
/* 
for element in &collection { ... }: items are taken as shared references
for element in &mut collection { ... }: items are taken as mutable references
for element in collection { ... }: items are moved out of the collection (which gets invalidated afterwards) 
*/
fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
//-----------------------------------------------Custom Iterator-----------------------------------------------------------------------  
struct Twice {
    count: u32,
    element: u32,
}
fn twice(element: u32) -> Twice {
    Twice { 
        count: 0, 
        element, 
    }
}
impl Iterator for Twice {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 2 {
            None 
        } else {
            self.count += 1;
            Some(self.element)
        }
    }
}

fn main() {
//---------------------------------------Iterator Pipeline-----------------------------------------------------------------------  
    let r1 = 0..=5; // (1..=n) including, (1..n) excluding
    for element in r1 { // use for to consume iterator
        println!("{}", element);
    }
    // above similar to below
    let r2 = 0..=5;
    let mut iterator = (r2).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
//---------------------------------------Iterator Producer-------------------------------------------------------------
    for i in fib_iter(7) {
        println!("{}", i);
    }

    let once = std::iter::once(42);
    for item in once {
        println!("{}", item);
    }
    let repeat = std::iter::repeat(5). take(3);
    for item in repeat {
        println!("{}", item);
    }

    // We do not have to .take(n) this apparently infinite iterator. 
    // Since checked_mul returns None when the type (u8, from 0 to 255, in this case) would overflow.
    let pow_of_2_pipeline = std::iter::successors(Some(1u8), |n| n.checked_mul(2));
                                                                            // start with 1, which is assigned to type u8
    for item in pow_of_2_pipeline {
        println!("{}", item);
    }
//---------------------------------------Iterator Adapter-------------------------------------------------------------
    let sequence = std::iter::successors(Some(1u8), |n| n.checked_mul(2))
        .skip(2) // skip 2 items ban đầu, i.e. 1, 2
        .take(3); // take 3 items trong sequence, i.e. 4, 8, 16
    for item in sequence {
        println!("{}", item);
    }

    let pow_of_2_adapter = (2..=5).map(|n| 2_i32.pow(n)); // map với 2^2, 2^3, 2^4, 2^5
    for item in pow_of_2_adapter {
        println!("{}", item);
    }

    let odd_squares = (0..10).filter_map(|n|
        if n%2 == 1 { // chỉ map số lẻ với n*n
            Some(n*n)
        } else {    
            None 
        });
    for item in odd_squares {
        println!("{}", item);
    }   

    use std::collections::BTreeMap;
    let mut comics_adapter = BTreeMap::new();
    comics_adapter.insert("Peanuts", vec!["Charlie", "Linus", "Lucy", "Snoppy"]);
    comics_adapter.insert("Calvin & Hobbes", vec!["Calvin", "Hobbes", "Susie"]);
    let all_characters: Vec<_> = comics_adapter
        .values() 
            // 2 values in the BTree: (1)["Calvin", "Hobbes", "Susie"], (2)["Charlie", "Linus", "Lucy", "Snoppy"]
        .inspect(|value| {println!("Before {:?}", value);})
            // inspect = looking at each item (value) immutably, e.g. print it
            // -> Before ["Calvin", "Hobbes", "Susie"]
        .flatten() 
            // tách các element của từng value ra
        .inspect(|value| {println!("After: {}", value);})
            // inspect = looking at each item (element) immutably, e.g. print it
            // -> After: Calvin
            //    After: Hobbes
            //    After: Susie
        .collect();
            // transform an iterator into a collection
            // -> ["Calvin", "Hobbes", "Susie", "Charlie", "Linus", "Lucy", "Snoppy"]
    println!("All: {:?}", all_characters);

    let range = (0..5).chain(10..14);
    for item in range {
        println!("{}", item);
    }

    for (i, item) in (5..10).enumerate() {
        println!("{}th: {}", i, item); // (index of the element in the iterator, element)
    }
//---------------------------------------Iterator Consumer-------------------------------------------------------------
    let n = 5;
    println!("Triangle {}: {}", n, triangle(n));
    println!("Factorial {}: {}", n, factorial(n));

    println!("Max: {:?}", [-7, 5, 0, 28, -2].iter().max());
    println!("Min: {:?}", [-7, 5, 0, 28, -2].iter().min());

    let a = [1, 2, 3, 4, 5];
    println!("Sum: {}", a.iter().fold(0, |n, i| n+i)); // 0+1+2+3+4+5
    println!("Product: {}", a.iter().fold(1, |n, i| n*i)); //1*1*2*3*4*5

    use std::collections::HashMap;
    let comics_consumer = ["Peanuts", "Calvin and Hobbes"];
    let start_dates = [1950, 1985];
    let start_dates = comics_consumer
        .iter() // returns an iterator over the slice
        .zip(start_dates.iter()) // zip two iterators into a single iterator of pairs
        .collect::<HashMap<_,_>>(); // turbofish ::<> operator, _ is used to run type inference (key: &str, value: i32)
    println!("{:?}", start_dates);
//---------------------------------------Custom Iterator-----------------------------------------------------------------------  
    let t = twice(5);
    let c = t.collect::<Vec<_>>();
    println!("{:?}", c);
    assert_eq!(c, vec![5,5]);
}

//=============Closures: anonymous functions you can save in a variable or pass as arguments to other functions===================================================================

/* fn main() {
//---------------------------------------Closure Explained-----------------------------------------------------------------------  
    let outer_var = 42;
    /* fn function                                             (i: i32) -> i32 {i + outer_var} 
    // can't refer to variables in the enclosing environment */
    let closure_annotated            = |i: i32| -> i32 {i + outer_var};
    let closure_inferred             = |i|        {i + outer_var};
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let one = || {1};
    println!("closure returning one: {}", one());
    
    let pow_of_2_closure = std::iter::successors(Some(1u8), 
        |n| n.checked_mul(2) // <- closure
    );   
    for item in pow_of_2_closure { // pow_of_2_closure have no argument
        println!("{}", item);
    }

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("Ferris", 42);
    map.entry("Carb").or_insert_with(|| 47); 
                        // ensures a value is in the entry by inserting the result of the default function if empty
                        // function is fn function () -> i8 { return 47; } in this case
    println!("{:#?}", map);

    for i in fib_iter(7){
        println!("{}", i);
    }
//---------------------------------------Closure Types-----------------------------------------------------------------------  
/* 
Closure that drops something it stole (self) from the environment (i.e. variable v).
This closure implements trait "FnOnce" because it can only be called once, otherwise, would cause double-free error
trait FnOnce() -> R {
    fn call_once(self) -> R;
} 
*/
    let v: Vec<u32> = vec![];
    let steal_closure = || drop(v); 

/* 
Closure that only modifies (&mut self) the environment.
This closure implements "FnMut", as it can mutate the environment 
trait FnMut() -> R {
    fn call_mut(&mut self) -> R;
}
*/
    let mut i = 0;
    let mut incr = || {
        i += 1;
        println!("Increment! i is now {}", i);
    };
    incr();
    incr();

/* 
Closure that only reads (&self) from the environment.
This closure implement Fn
trait Fn() -> R {
    fn call_mut(&self) -> R;
}
*/
    let r = 23;
    let read_closure = || -> i32 {r + 2};
    println!("Read value 23 is increased by 2: {}", read_closure());

/*     
Closure that move/steal the environment (i.e. variable greeting).
If a closure only holds references and does not mutate (Fn), it can be copied and cloned.
It depends on the type of values that are moved into the closure,
if they are all clone or copy, the closure is clone or copy, respectively
*/
    let mut greeting = String::from("Hello, ");
    let greet = move |name| { // "move" = the closure want to move/steal "greeting" variable  
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Ferris"); // closure "greet" is cloned due to variable "greeting" is cloned
    greet.clone()("Hobbes");

} */

//=========================Collections: help working with multiple items at the same time===================================================================

/* fn main() {
//------------------------------------------Vector: Vec<T>-----------------------------------------------------------------------  
    struct Point {
        x: u8,
        y: u8
    }
    let points: Vec<Point> = Vec::new();
    let mut points: Vec<Point> = vec![Point {x: 0, y: 1}, Point {x: 2, y: 3}];
    points.push(Point { x: 0, y: 0 });

    /* let mut v = vec![0,2,4,6];
    let mut iterator = (v).into_iter(); // or &v, Rust prevent the infinte loop unlike Java
    while let Some(element) =  iterator.next() {
        v.push(element+1);
    } */

//--------------------------------------Dictionary: HashMap<K,V>----------------------------------------------------------------------- 
    use std::collections::HashMap;

    let key_values = vec![(7, 'H'), (-3, 'e')];
    let mut map : HashMap<_,_> = key_values.into_iter().collect(); // .into_iter() break elements, .collect() elements into HashMap
    map.insert(42, 'H');
    println!("{:#?}", map);
    for (k,v) in map {
        println!("K: {}, V: {}", k, v);
    }

    let mut letters = HashMap::new();
    for ch in "a practical course for computer scientist".chars() {
        // if that element hasn't been added into the HashMap, add it and intialize the value of element to 0
        // if that element has been added into the HashMap, take the return value of element and +1
        let counter = letters.entry(ch).or_insert(0); 
        *counter += 1;
    }
    println!("{:#?}", letters);

//-----------------------------------------Set: HashSet<T>----------------------------------------------------------------------- 
    use std::collections::HashSet;

    let mut set : HashSet<_> = [4,5,4].into_iter().collect(); // duplicates are removed
    set.insert(5);
    set.insert(8);
    println!("{:#?}", set);
    set.extend(vec![7,5,3].into_iter());
    println!("{:#?}", set);

    let setA : HashSet<_> = [1,2,3].into_iter().collect();
    let setB : HashSet<_> = [2,3,4].into_iter().collect();
    for i in setA.intersection(&setB) {
        print!("{}", i);
    }
    println!("");
    for i in setA.union(&setB) {
        print!("{}", i);
    }
    println!("");
    for i in setA.difference(&setB) {
        print!("{}", i);
    }
    println!("");

//---------------------------------------------BTrees-----------------------------------------------------------------------  
    use std::collections::BTreeMap;

    let mut comics_adapter = BTreeMap::new();
    comics_adapter.insert("Peanuts", vec!["Charlie", "Linus", "Lucy", "Snoppy"]);
    comics_adapter.insert("Calvin & Hobbes", vec!["Calvin", "Hobbes", "Susie"]);
    let all_characters: Vec<_> = comics_adapter
        .values()
        .inspect(|value| {println!("Before {:?}", value);})
        .flatten()
        .inspect(|value| {println!("After: {}", value);})
            // inspect a pipeline by looking at each item immutably, e.g., to print it
        .collect();
    println!("All: {:?}", all_characters);
} */

//================================================Strings===================================================================
/* fn main() {
    let s = String::new();
    let s = "Hey Khue".to_string();
    println!("{}", s);
    let s = String::from_utf8(vec![0xF0, 0x9F, 0xA6, 0x80]);
    println!("{:#?}", s);
    let mut s : String = vec!["Hey", "Khue"].into_iter().collect();
    println!("{}", s);
    s.push_str("!");
    println!("{}", s);

    let string = "Hello Khue. How are you doing?";
    let index = string.find("are"); // white space also get counted
    println!("{:#?}", index);
    println!("{}", string.replace("Khue", "Az"));

    let file_content = "Id,Name\n42,Khue\n49,Az";
    for element in file_content // elements gồm: (1st-iter)Id, (2nd-iter)Name\n42, (3rd-iter)Khue\n49, (4th-iter)Az
        .lines()// Spilt thành từng line (1)Id,
                                                // (2)Name
                                                // (2)42,
                                                // (3)Khue
                                                // (3)49,
                                                // (4)Az
        .flat_map(|line| line.split(",")) // xoá dấu "," ở từng line
            .collect::<Vec<_>>() {  // collect lại toàn bộ rồi print từng element đã biến thành line và bỏ dấu ","
                println!("{}", element);
            }

    //{which: how}
    println!("{:+}", 108); // forced sign
    println!("{:10}", 108); //minimum field width
    println!("{:010}", 108); // minimum field width, leading zeros
    println!("{:02x}", 108); // hexadecimal
    println!("{:02x?}", [108, 11, 42]); 
    println!("{:12.2}", 1234.5678); // float formatting
    println!("{:10}", "Khue"); // minimal field width
    println!("{:.5}", "Hello Khue"); // text length limit
    println!("{:>20}", "Hello Khue"); // alignment
    println!("{:=^20}", "Khue"); // padding + center
    let data = std::rc::Rc::new("Khue".to_string());
    println!("{:p}", data); // pointer
} */
