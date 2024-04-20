extern crate rand;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

struct Word {
    answer: String,
    correct_count: usize,
    representation: String,
    length: usize,
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
        let mut found: bool = false;
        let mut response = String::with_capacity(self.length); // Используем with_capacity, который позволяет изначально выделить всю требуемую память
        let mut index: i32 = 0;

        for letter in self.answer.chars() { // Метод .chars() возвращаем нам объект итератор, по которому мы можем пройтись циклом for (Каждый символ будет отдельным элементом)
            if letter == char {
                found = true; // Говорим что 1 символ найден
                count += 1; // Увеличиваем счётчик угаданных символов

                response.push(char); // Добавляем угаданный символ
            }
            else {
                if self.representation.chars().nth(index) != Some("_") { // Проверяем было ли ранее угадано число
                    response.push(self.representation.chars().nth(index).unwrap()); // Добавляем угаданный символ
                }
                else {
                    response.push("_"); // Добавляем _ для обозначения того, что мы не угадали правильный символ
                }
            }

            index += 1;
        }

        self.representation = response; // Присваиваем новое значение в угадывающуюся строку
        self.correct_count += count;
        count > 0 // Возвращаем булево значение, которое говорит - угадали ли мы хотя бы один символ
    }
}
