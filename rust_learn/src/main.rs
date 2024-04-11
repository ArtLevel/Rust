use std::io;

// const PI: f64 = 3.14; // Можно объявить константы

// fn main() {
//     let a = 42; // immutable, переменная не изменяется в ходе выполнения кода

//     let mut b = 123; // mutable, переманная может изменятся в ходе выполнения кода
//     let b2 = b;

//     println!("Hello, world! The value: {b}");
//     b = 321; // mutable, переманная может изменятся в ходе выполнения кода
//     println!("Hello, world! The value: {b}");
   
//     // Types
//     // Scalar
//     // 1. Integer, целочисленные числа
//     // u8 максимумальное число 255, потому-что: [0, (2 ** 8) - 1] == [0, 255]

//     let mut c: i8;
//     // u = unsigned, число без знака
//     // c = -5; ERROR

//     // i = signed, число с знаком

//     let d: i128;
//     // let d: usize; // arch

//     // 0o - octal
//     // 0b - binary

//     // 2. Float, числа с плавающей запятой
//     // f32
//     // f64

//     let float = 42.5;

//     // Operations
//     // +
//     // -
//     // *
//     // /

//     // 3. Boolean
//     // true
//     // false

//     let flag = true;

//     // 4. Char, символ
//     let ch: char = 'a'; // 4 bytes
//     let ch2: char = 'ы'; // 4 bytes

//     // COMPOUND
//     // 1. Tuple, кортеж
//     let my_tuple: (i8, char, bool) = (42, 'f', false);
//     let tuple_int = my_tuple.0; // Достаём из кортежа число
    
//     // 2. Array
//     let arr = [1, 2, 3]; // Fixed Array
//     arr[1];
//     // arr[435]; ERROR


// }

// В Rust много знакомых типов:
//
//     булевый - bool представляет true/false
//     без знаковые целочисленные - u8 u16 u32 u64 u128 представляют только положительные числа
//     знаковые целочисленные - i8 i16 i32 i64 i128 представляют положительные и отрицательные числа
//     целочисленные для размеров указателей - usize, isize для представления индексов и размеров элементов в памяти
//     числа с плавающей точкой - f32 f64
//     кортежи - (значение,значение,...) представляют фиксированную последовательность из разных типов значений, размещаются на стеке
//     массивы - [значение, значение, ...] представляют последовательность элементов одного типа, имеющий фиксированный размер известный на стадии компиляции.
//     срезы (slices) - коллекция одинаковых элементов с длиной, известной во время выполнения
//     строковый срез str ( string slice ) - текст с размером определяемым во время выполнения
// fn greeting ()
// {
//     print!("Hello Everyone !");
//     let sum = 20u8 + 3u16 as u8;
//     let sum = 20u8 + 3u16 as u8;
//     let sum = 20u8 + 3u16 as u8;
//     let sum = 20u8 + 3u16 as u8;
//     let sum = 20u8 + 3u16 as u8;
//     let sum = 20u8 + 3u16 as u8;
//     print!("{}", sum);
//
//     let x = 42;
//
//     match x {
//         0 => {
//             println!("found zero");
//         }
//         // we can match against multiple values
//         1 | 2 => {
//             println!("found 1 or 2!");
//         }
//         // we can match against ranges
//         3..=9 => {
//             println!("found a number 3 to 9 inclusively");
//         }
//         // we can bind the matched number to a variable
//         matched_num @ 10..=100 => {
//             println!("found {} number between 10 to 100!", matched_num);
//         }
//         // this is the default match that must exist if not all cases are handled
//         _ => {
//             println!("found something else!");
//         }
//     }
//
//     let mut y = 0;
//
//     let v = loop {
//         y += 1;
//         if y == 13 {
//             break "found the 13";
//         }
//     };
// asdfasdfasdf fsadf sadfasdf
//     println!("from loop: {}", v);
//
// }
//
// // Tutor Of Rust
//
// fn main ()
// {
//     asdfasdfasdfasdfasdad asdasd
//     let a = 10;
//
//     let result = greeting(); // пустой кортеж, также назвыают это как unit (юнит тип)
//     result.0;
//     println!("{}", result);
//
//     println!("{} {}", x = 10, a);
// }

// fn example() -> i32 {
//     let x = 42;
//     // Rust's ternary expression
//     let v = if x < 42 { -1 } else { 1 }; // 1
//     println!("from if: {}", v);
//
//     let food = "hamburger";
//     let result = match food {
//         "hotdog" => "is hotdog",
//         // notice the braces are optional when its just a single return expression
//         _ => "is not hotdog",
//     };
//     println!("identifying food: {}", result); // "is not hotdog"
//
//     let v = {
//         // This scope block lets us get a result without polluting function scope
//         let a = 1;
//         let b = 2;
//         a + b
//     };
//     println!("from block: {}", v); // 3
//
//     // The idiomatic way to return a value in rust from a function at the end
//     v + 4 // 7
// }
//
// fn main() {
//     println!("from function: {}", example());
// }

// struct SeaCreature {
//     animal_type: String,
//     name: String,
//     arms: i32,
//     legs: i32,
//     weapon: String,
// }
//
// fn main() {
//     // SeaCreature's data is on stack
//     let ferris = SeaCreature {
//         // String struct is also on stack,
//         // but holds a reference to data on heap
//         animal_type: String::from("crab"),
//         name: String::from("Ferris"),
//         arms: 2,
//         legs: 4,
//         weapon: String::from("claw"),
//     };
//
//     // let dora = SeaCreature {
//     //     animal_type: String::from("fish"),
//     //     name: String::from("Dora"),
//     //     arms: 0,
//     //     legs: 0,
//     //     weapon: String::from("none")
//     // };
//
//     let sarah = SeaCreature {
//         animal_type: String::from("octopus"),
//         name: Straing::from("Sarah"),
//         arms: 8,
//         legs: 0,
//         weapon: String::from("none"),
//     };
//
//     println!(
//         "{} is a {}. They have {} arms, {} legs, and a {} weapon",
//         ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
//     );
//     println!(
//         "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
//         sarah.name, sarah.animal_type, sarah.arms, sarah.legs
//     );
//     // println!(
//     //     "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
//     //     dora.name, dora.animal_type, dora.arms, dora.legs
//     // );
// }
//

// struct Numbers (i32, bool);
//
// fn main ()
// {
//     let num1 = Numbers(32, true);
//     let num2 = Numbers(-53, false);
//
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num2.0, num2.1);
// }
//
// struct Numbers (i32, bool);
//
// fn main ()
// {
//     let num1 = Numbers(32, true);
//     let num2 = Numbers(-53, false);
//
//     println!("{} {}", num1.0, num1.1);
//     println!("{} {}", num2.0, num2.1);
// }

// #![allow(dead_code)] // this line prevents compiler warnings

// enum Species { Crab, Octopus, Fish, Clam }
// enum PoisonType { Acidic, Painful, Lethal }
// enum Size { Big, Small }
// enum Weapon {
//     Claw(i32, Size),
//     Poison(PoisonType),
//     None
// }
// enum Eyes {
//     Black,
//     White,
//     Light
// }
// struct SeaCreature {
//     species: Species,
//     name: String,
//     arms: i32,
//     legs: i32,
//     weapon: Weapon,
//     eyes: Eyes
// }
//
// fn main() {
//     // SeaCreature's data is on stack
//     let sarah = SeaCreature {
//         // String struct is also on stack,
//         // but holds a reference to data on heap
//         species: Species::Octopus,
//         name: String::from("Sarah"),
//         arms: 8,
//         legs: 0,
//         weapon: Weapon::Poison(PoisonType::Acidic),
//         eyes: Eyes::Light,
//     };
//
//     match sarah.species {
//         Species::Octopus => {
//             match sarah.weapon {
//                 Weapon::Poison(PoisonType) => {
//
//                     let poison_description = match PoisonType {
//                         PoisonType::Painful => "Painful",
//                         PoisonType::Lethal => "Lethal",
//                         PoisonType::Acidic => "Acidic",
//                     };
//                     let eyes = match sarah.eyes {
//                         Eyes::Light => "Light",
//                         Eyes::Black => "Black",
//                         Eyes::White => "White",
//                     };
//
//                     println!("ferris is a octopus with poison {}. Ferris has {} eyes", poison_description, eyes)
//
//                 },
//                 _ => println!("ferris is a crab with some other weapon")
//             }
//         },
//         _ => println!("sarah is some other animal"),
//     }
// }

// A partially defined struct type
// struct BagOfHolding<T> {
//     item: T,
// }
//
// fn main() {
//     // Note: by using generic types here, we create compile-time created types.
//     // Turbofish lets us be explicit.
//     let i32_bag = BagOfHolding::<i32> { item: 42 };
//     let bool_bag = BagOfHolding::<bool> { item: true };
//
//     // Rust can infer types for generics too!
//     let float_bag = BagOfHolding { item: 3.14 };
//
//     // Note: never put a bag of holding in a bag of holding in real life
//     let mut bag_in_bag = BagOfHolding {
//         item: BagOfHolding { item: "boom!" },
//     };
//
//     bag_in_bag.item.item = "HELLO";
//
//     println!(
//         "{} {} {} {}",
//         i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
//     );
// }

// A partially defined struct type
// struct BagOfHolding<T> {
//     // Our parameter type T can be handed to others
//     item: Option<T>,
// }
//
// fn main() {
//     // Note: A bag for i32, holding nothing! We have to specify the type
//     // because otherwise Rust would not know what type of bag it is.
//     let i32_bag = BagOfHolding::<f64> { item: None };
//
//     if i32_bag.item.is_none() {
//         println!("there's nothing in the bag!")
//     } else {
//         println!("there's something in the bag!")
//     }
//
//     let i32_bag = BagOfHolding::<bool> { item: Some(42) };
//
//     if i32_bag.item.is_some() {
//         println!("there's something in the bag!")
//     } else {
//         println!("there's nothing in the bag!")
//     }
//
//     // match lets us deconstruct Option elegantly and ensure we handle all cases!
//     match i32_bag.item {
//         Some(v) => println!("found {} in bag!", v),
//         None => println!("found nothing"),
//     }
// }

// fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
//     if i == 42 {
//         Ok(13.0)
//     } else {
//         Err(String::from("this is not the right number"))
//     }
// }
//
// fn main() -> Result<(), String> {
//     // concise but assumptive and gets ugly fast
//     let v = do_something_that_might_fail(42).unwrap(); // Result<T, E> / Option<T>
//     println!("found {}", v);
//
//     // this will panic!
//     let v = do_something_that_might_fail(1).unwrap();
//     println!("found {}", v);
//
//     Ok(())
// }

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let foo = Foo { x: 42 };
    // foo is moved to do_something
    do_something(foo);
    // foo can no longer be used

    // println!("{}", foo.x); Error
}
