mod chapter1; // mod는 해당 스크립트 파일을 모듈로서 가져올 수 있게 해준다. (내부 pub 정의된 애들이 export처럼 들고올 수 있음.)
mod chapter2;

fn main() {
    println!("would this be compiled? {}", "hello");

    let mut target_string = String::from("공백이 존재합니다.");
    let word_length = chapter2::first_word(&target_string);

    // target_string = String::from("hi");
    // target_string.clear();

    println!("word length is {}", word_length);
}
