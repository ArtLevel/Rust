extern crate rand; // импортируем сторонний контейнер, функиця rand генерирует булевое значение
use std::thread; // импортируем контейнер, который входит в стандартную библиотеку
use std::time; // импортируем контейнер, который входит в стандартную библиотеку

// mut - переменная может мутировать

// let i_love_rust = "❤"; // строка
// let array = [10u8; 10]; // простой массив
// let yes: bool = true; // булевое значение
// let mut count = 1110;

// let i: i32;
// let b: bool;
//
// (i, b) = function1(); // кортеж

fn census(_world: [[u8; 100]; 100]) -> u16 // Функция по подсчёту всех живых слеток
{

    let mut count = 0;

    for i in 0..99 {
        for j in 0..99 {
            if _world[i][j] == 1 { // если в ячейки есть жизнь, то увеличиваем счётчик
                count += 1;
            }
        }
    }

    count // возвращаем счётчик
}

fn generation(world: [[u8; 100]; 100]) -> [[u8; 100]; 100] // 1 Итерация/Ход в мире
{
    let mut new_world = [[0u8; 100]; 100]; // создаём новый мир

    for i in 0..99 {
        for j in 0..99 {
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
            if i < 99 {
                count = count + world[i + 1][j]
            }

            if j < 99 {
                count = count + world[i][j + 1]
            }

            if j != 0 {
                count = count + world[i][j - 1]
            }

            if i != 0 {
                count = count + world[i - 1][j]
            }

            if i < 99 && j != 0 {
                count = count + world[i + 1][j - 1]
            }

            if i != 0 && j != 0 {
                count = count + world[i - 1][j - 1]
            }

            if j < 99 && i != 0 {
                count = count + world[i - 1][j + 1]
            }

            if j < 99 && i < 99 {
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
    // [0u8 <- u8 битные числа будут в массиве, и все 100 чисел будут равны 0; 100 <- размер массива]
    let mut world = [[0u8; 100]; 100]; // игровая матрица, многомерный массив
    let mut generations = 0; // счётчик поколений

    for i in 0..99 { // присваиваем i = 0, и делаем условие i < 74; i++
        for j in 0..99 { // i колонка, j столбец
            if rand::random() {
                world[i][j] = 1;
            } else {
                 world[i][j] = 0;
            }
        }
    }

    let new_world: [[u8; 100]; 100] = generation(world);
}
 