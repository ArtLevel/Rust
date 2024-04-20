// Подключаем нужные контейнеры
extern crate rand;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

struct Word { // Объявляем структуру Word (Слово)
    correct_count: usize,
    representation: String,
    length: usize,
    answer: String,
}

trait CheckLetter { // типаж который описывает функцию check_for_letter
    fn check_for_letter(&mut self, c: char) -> bool;
}

trait CheckComplete { // типаж который описывает функцию check_complete
    fn check_complete(&self) -> bool;
}

impl CheckComplete for Word { // impl - это реализация наших типажей (check_complete)
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }
}

impl CheckLetter for Word { // impl - это реализация наших типажей (check_for_letter)
    fn check_for_letter(&mut self, char: char) -> bool {
        let mut count: usize = 0;
        let mut _found: bool = false;
        let mut response = String::with_capacity(self.length); // Используем with_capacity, который позволяет изначально выделить всю требуемую память
        let mut index: usize = 0;

        for letter in self.answer.chars() { // Метод .chars() возвращаем нам объект итератор, по которому мы можем пройтись циклом for (Каждый символ будет отдельным элементом)
            if letter == char {
                _found = true; // Говорим что 1 символ найден
                count += 1; // Увеличиваем счётчик угаданных символов

                response.push(char); // Добавляем угаданный символ
            }
            else {
                if self.representation.chars().nth(index) != Some('_') { // Проверяем было ли ранее угадано число
                    response.push(self.representation.chars().nth(index).unwrap()); // Добавляем угаданный символ
                }
                else {
                    response.push('_'); // Добавляем _ для обозначения того, что мы не угадали правильный символ
                }
            }

            index += 1;
        }

        self.representation = response; // Присваиваем новое значение в угадывающуюся строку
        self.correct_count += count; // Записываем корректное значение угаданных символов
        count > 0 // Возвращаем булево значение, которое говорит - угадали ли мы хотя бы один символ
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> // Функция, которая возвращает итератор по всем линиям переданного файла
where P: AsRef<Path>, { // Where ключевое слово для добавления специфичности типу P. Далее мы используем типаж AsRef<Path> для того, чтобы преобразовать String в Path
    let file = File::open(filename)?; // ? - оператор, который позволяет элегантно обработать ошибку (Используется на типе Result/Option, в данном случае Result)

    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: String) -> Vec::<String> { // Функция для чтения списка слов
    let mut v = Vec::<String>::new(); // Создаём вектор строк

    if let Ok(lines) = read_lines(filename) { // Создаём переменную lines и проверяем её значение
        for w in lines {
            let word: String = w.unwrap();

            if word.len() > 4 { // Проверка на длину, чтобы слова не были слишком короткими (Вес строки должен быть не меньше чем 4 байта)
                v.push(word);
            }
        }
    }

    v // Возвращаем вектор строк
}

fn select_word() -> String {
    let mut rng = rand::thread_rng(); // Для генерации случайных чисел
    let filename: String = "words.txt".to_string(); // Имя файла (.to_string() приводит тип Строковой Литерал к типу String)
    let words: Vec<String> = read_list(filename); // Получаем Вектор Строк в котором все наши слова
    let word_count = words.len(); // Берём количество слов в файле (.len() возвращает количество байт)

    let selection = rng.gen_range(1, word_count); // Генерируем число в определённом диапазоне, от 1 до word_count
    let select: String = words[selection].clone(); // Выбираем определённое слово из Вектора Строк
    // .clone() мы используем чтобы скопировать значение из ячейки памяти, где храниться наше слово. Мы не можем заимствовать владение от определённого элемента в Vector, так как им уже владеет Vector

    select // Возвращаем выбранное слово
}

fn main() {
    let body: Vec<String> = vec![
        "noose".to_string(),
        "head".to_string(),
        "neck".to_string(),
        "torso".to_string(),
        "left arm".to_string(),
        "right arm".to_string(),
        "right leg".to_string(),
        "left leg".to_string(),
        "left foot".to_string(),
        "right foot".to_string()
    ]; // Части тела

    let mut body_iter = body.iter(); // Итератор по частям тела
    let mut result = select_word(); // Загаданное слово

    let mut answer = Word {
        length: result.len(),
        representation: String::from_utf8(vec![b'_'; result.len()]).unwrap(), // Заполняем изначальное слово _ (_ каждый символ)
        answer: result,
        correct_count: 0
    }; // Инициализируем экземпляр структуры Word

    let mut letter: char; // Символ
    let mut body_complete: bool = false; // Когда конец игры

    while !answer.check_complete() && !body_complete { // Запускаем процесс игры
        println!("Provide a letter to guess ");
        let mut input = String::new(); // То, что ввёл пользователь

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                letter = input.chars().nth(0).unwrap();
                if answer.check_for_letter(letter) { // Проверяем является ли буква правильной
                    println!("There is at least one {}, so the word is {}", letter, answer.representation);
                }
                else {
                    let next_part = body_iter.next().unwrap(); // Идём по итератору, который берёт значения из body (Части тела, строковой вектор)
                    println!("Incorrect! You are at {}", next_part);
                    if next_part == "right foot" { // Если мы не смогли отгадать слово и попытки закончились
                        body_complete = true;
                    }
                }
            },
            Err(_error) => {
                println!("Didn't get any input"); // Даём игроку знать что он ввёл что-то неправильно
            }
        }
    }

    if body_complete { // Если игрок не отгадал слово
        println!("You were unsuccessful at guessing {}", &answer.answer);
    }
    else { // Если игрок отгадал слово
        println!("Yes! The word was {}", &answer.answer);
    }
}
