extern crate rand; // импортируем сторонний контейнер, функиця rand генерирует булевое значение
extern crate termion; // импортируем сторонний контейнер, termion поможет нам при работе с командной строкой
use std::{env, thread, time}; // импортируем контейнер, который входит в стандартную библиотеку
use std::fs::File; // импортируем контейнер, который входит в стандартную библиотеку
use std::io::{BufRead, BufReader}; // импортируем контейнер, который входит в стандартную библиотеку
use termion::clear;
use termion::color;

// mut - переменная может мутировать

// let i_love_rust = "❤"; // строка
// let array = [10u8; 10]; // простой массив
// let yes: bool = true; // булевое значение
// let mut count = 1110;

// let i: i32;
// let b: bool;
//
// (i, b) = function1(); // кортеж

fn census(_world: [[u8; 250]; 75]) -> u16 // Функция по подсчёту всех живых слеток
{

    let mut count = 0;

    for i in 0..74 {
        for j in 0..249 {
            if _world[i][j] == 1 { // если в ячейки есть жизнь, то увеличиваем счётчик
                count += 1;
            }
        }
    }

    count // возвращаем счётчик
}

fn generation(world: [[u8; 250]; 75]) -> [[u8; 250]; 75] // 1 Итерация/Ход в мире
{
    let mut new_world = [[0u8; 250]; 75]; // создаём новый мир

    for i in 0..74 {
        for j in 0..249 {
            let mut count = 0; // количество соседей у клетки

            // Подсчитываем количество соседей у клетки
            // if i > 0 {
            //     count = count + world[i][j];
            // }
            //
            // if i > 0 && j > 0 {
            //     count = count + world[i - 1][j - 1];
            // }
            //
            // if i > 0 && j < 74 {
            //     count = count + world[i - 1][j + 1];
            // }
            //
            // if i < 74 && j > 0 {
            //     count = count + world[i + 1][j - 1];
            // }
            //
            // if j > 0 {
            //     count = count + world[i][j - 1];
            // }
            //
            // if j < 74 {
            //     count = count + world[i][j + 1];
            // }

            // Подсчитываем количество соседей у клетки
            if i < 74 {
                count = count + world[i + 1][j]
            }

            if j < 249 {
                count = count + world[i][j + 1]
            }

            if j != 0 {
                count = count + world[i][j - 1]
            }

            if i != 0 {
                count = count + world[i - 1][j]
            }

            if i < 74 && j != 0 {
                count = count + world[i + 1][j - 1]
            }

            if i != 0 && j != 0 {
                count = count + world[i - 1][j - 1]
            }

            if j < 249 && i != 0 {
                count = count + world[i - 1][j + 1]
            }

            if j < 249 && i < 74 {
                count = count + world[i + 1][j + 1]
            }

            new_world[i][j] = 0;

            if (count < 2) && (world[i][j] == 1) { // Если у живой клетки нет соседей, то клетка умирает
                new_world[i][j] = 0;
            }

            if world[i][j] == 1 && (count == 2 || count == 3) { // Если клетка жива и у неё 2 или 3 соседа, то клетка проживает ещё 1 поколение
                new_world[i][j] = 1;
            }

            if (world[i][j] == 0) && (count == 3) { // Если клетка мертва, но у неё 3 соседа, то клетка рождается заново
                new_world[i][j] = 1;
            }

        }
    }

    new_world // возвращаем новый мир
}

fn main () // создаём мир
{
    // [0u8 <- u8 битные числа будут в массиве, также в массиве будут 75 чисел; 75 <- размер массива]
    let mut world = [[0u8; 250]; 75]; // игровая матрица, многомерный массив
    let mut generations = 0; // счётчик поколений

    let args: Vec<String> = env::args().collect(); // Вектор - это набор единственного типа данных. В данном случае это набор строк
    // args() функция,которая возвращает все аргументы из командной строки, пример: ./life filename
    // collect() превращает вектор в коллекцию

    if args.len() < 2 { // Если не передали в консоли world, то генерируем world случайным образом
        for i in 0..74 { // присваиваем i = 0, и делаем условие i < 74; i++
            for j in 0..249 { // i колонка, j столбец
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap(); // Используем unwrap для извлечения String из Option
        world = populate_from_file(filename) // Option имеет два значения: 1. Ничто (None), 2. Some, some ожидает что тип будет определён
    }
    // let str: Option<String> = None; // Нужно разобраться с Option

    for _gens in 0..1000 { // Делаем цикл на 1000 поколений
        let temp = generation(world); // Делаем 1 ход поколения
        world = temp; // Перезаписываем адрес temp на world
        generations += 1; // увеличиваем счётчик поколений

        println!("{}", clear::All); //Очищаем всё содержимое терминала
        display_world(world); // Отображаем мир
        println!("{blue}Поколения: {g} Численность населения: {c}", blue = color::Fg(color::Blue), g = generations, c = census(world)); // Выводим дополнительную информацию

        thread::sleep(time::Duration::from_millis(250)); // Делаем небольшую задержку между ходами поколений
    }

    // let var1 = 42; // В Rust на 1 ячейку памяти ссылается 1 индентификатор (адрес)
    // let var2;
    // var2 = var1; // Здесь мы переприсваиваем индентификатор с var1 на var2. После этого, var1 не существует

    // let new_world: [[u8; 75]; 75] = generation(world);
}

fn populate_from_file (filename: String) -> [[u8; 250]; 75] // Функция которая генерирует мир на основе файла пользователя
{
    let mut new_world = [[0u8; 250]; 75]; // создаём новый мир
    let file = File::open(filename).unwrap(); // Открывам переданный файл, используем unwrap для этой задачи. Нам возвращается тип данных Result,у result 2 значения: 1. Ok, 2. Err
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new(); // Вектор из двух челых чисел. Каждая запись в векторе будет парным множеством целых чисел. Здесь мы считываем пары

    for(_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();

        let left = words.next().unwrap(); // строка, а в ней число
        let right = words.next().unwrap(); // строка, а в ней число

        let left_int = left.parse::<usize>().unwrap(); // Преобразуем строку в число, получаем Result. Чтобы получить настоящее число, используем unwrap
        let right_int = right.parse::<usize>().unwrap(); // Преобразуем строку в число, получаем Result. Чтобы получить настоящее число, используем unwrap

        pairs.push((left_int, right_int)); // Добавляем в виде пар целые числа в вектор
        // let left_int: usize = left.parse::<usize>().unwrap(); // Преобразуем левую строку в число
        // usize тип данных - означает, что размер целого числа будет привязан к архитектуре системы. Если вы находитесь на 64-разрядной системе, то usize будет 64-разрядным целым числом
    }

    for i in 0..74 { // Делаем мир, в котором все ячейки мертвы
        for j  in 0..249 {
            new_world[i][j] = 0;
        }
    }

    for (x, y) in pairs { // Заполняем мир, добавляем в ячейки жизнь
        new_world[x][y] = 1;
    }

    new_world // возвращаем новый мир, сгенерированный на основе файла пользователя
}

fn display_world(world: [[u8; 250]; 75]) // Отображаем нашу игру Жизнь на экране терминали/ консоли
{
    for i in 0..74 {
        for j in 0..249 {
            if world[i][j] == 1 {
                print!("{green}*", green = color::Fg(color::Green));
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
