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
fn greeting ()
{
    print!("Hello Everyone !");
    let sum = 20u8 + 3u16 as u8;
    let sum = 20u8 + 3u16 as u8;
    let sum = 20u8 + 3u16 as u8;
    let sum = 20u8 + 3u16 as u8;
    let sum = 20u8 + 3u16 as u8;
    let sum = 20u8 + 3u16 as u8;
    print!("{}", sum);

    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }

    let mut y = 0;

    let v = loop {
        y += 1;
        if y == 13 {
            break "found the 13";
        }
    };

    println!("from loop: {}", v);

}

// Tutor Of Rust

fn main ()
{
    asdfasdfasdfasdfasdad asdasd
    let a = 10;

    let result = greeting(); // пустой кортеж, также назвыают это как unit (юнит тип)
    result.0;
    println!("{}", result);

    println!("{} {}", x = 10, a);
}
