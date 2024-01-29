// crate란 러스트 코드의 더미와 같다 (like module)
// cargo는 npm처럼 프로젝트 생성 및 빌드, 실행등을 손쉽게 관리해주는 툴이다.
// cargo로 빌드를 했을 시, 빌드 대상은 target/debug에 자동 컴파일되어 저장되고
// 변경점이 있을 때에만 컴파일하며 변경점이 없으면 스마트하게 기존 파일을 실행시킨다.

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn chapter1() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("parsing error");
            return;
        }
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
