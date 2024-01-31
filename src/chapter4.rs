pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("empty inde is {}", i);
            return &s[..i];
        }
    }

    // 이해를 돕기 위한 예시 내용.
    let mut mutable_s = String::from("Hello World");
    let hello = &mutable_s[..5]; //가변 스트링의 0번째를 포인터로 가리키며, 길이는 5이다.
    let world = &mutable_s[6..]; //가변 스트링의 6번째를 포인터로 가리키며, 길이는 5이다.
    mutable_s.clear();

    // 만약 어떤 함수가 가변 스트링의 slice를 갖게 된다면 해당 가변 스트링은 불변으로 변하게 된다.
    // 이런 특성으로 인해, 이후 불변이 된 스트링을 가변 스트링처럼 사용하려고 할 경우(ex clear) 에러를  발생시키게 된다.

    // s.len()
    &s[..] //전체 슬라이스를 리턴함.
}

// 학습포인트
// 1. 포인터를 빌린 후 결과물(s.len())을 도출했을 때, 이것은 메모리 상 참조값 s와는 무관하게 생성된다.
// 2. 따라서, 이를 word_length 변수에 할당한 후 해당 가변 string을 clear 하더라도 문제가 되지 않는다.
// 3. 이는 string의 가변으로 인해 word의 length가 달라졌음에도 불구하고 싱크가 서로 맞지 않는 문제가 발생한다.

// 4. 러스트는 이런 문제를 스트링 슬라이스를 통해 해겷한다.
// 5. 반환형 usize는 스트링 길이에 해당하며, &str은 스트링 슬라이스를 의미한다.
// 6. 위에서 설명하였듯이, 가변 대상에 대해서 스트링 슬라이스를 할 경우, 대상은 불변한 대상으로 변한다.
// 7. 불변적 대상에 대해서 가변적인 행동을 하려 할 경우(ex, clear) 이는 컴파일러에 의해서 제지되게 된다.
// 8. 에러메세지는 이와 같다. "cannot assign to `target_string` because it is borrowed"
