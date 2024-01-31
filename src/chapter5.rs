// 구조체에 대한 이해.
// 구조체는 타입스크립트의 인스턴스, 자바의 클래스와 비슷한 느낌이다.
// 해당 구조체를 기반으로 인스턴스 생성이 가능하다.

// 참고로, 러스트에서는 함수 바깥에서 사용할 것 같은 값의 포인터는 상수로 처리할 것을 요구한다.
// 상수로 처리하게 될 경우, 해당 상수 포인터의 타입을 지정해줘야한다.

//const 상수로 메모리를 사용할 떄,
// &'static str을 통해서 해당 상수를 프로그래밍 토탈 라이프사이클동안 사용된다고 설정하고,
// 대문자로 네이밍을 해야 한다.
// struct UserConst {
//     username: &'static str,
//     email: &'static str,
//     age: u8,
// }

// const USERNAME: &'static str = "anderson";
// const EMAIL: &'static str = "test@abc.com";

// const USER_CONST: UserConst = UserConst {
//     username: USERNAME,
//     email: EMAIL,
//     age: 12,
// };

pub fn chapter5() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        age: u8,
    }

    #[derive(Debug)]
    struct UserByBuilder<'a> {
        username: &'a str,
        email: &'a str,
        age: u8,
    }

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    let mut user1 = User {
        username: String::from("anderson"),
        email: String::from("afweaef@afwefw.com"),
        age: 12,
    };

    // 구조체로 초기화된 인스턴스의 값을 조회하거나, 변경하는 일은 자바스크립트때와 비슷하다.
    // 단, let mut이 아닌 포인터일 경우, 해당 객체의 변경이 제한된다.
    // 러스트에서는 설령 힙에 저장되는 객체의 변경이 이루어지더라도 mut 키워드가 붙지 않으면 변경할 수 없다.
    user1.email = String::from("changed@email.com");

    // 구조체 갱신법을 이용하여 객체의 빈 부분을 채워넣는 것이 가능하다.
    // 단, 자바스크립트때와 다르게 기존의 프로퍼티 값이 덮어씌워지지 않고, struct에 비어있는 부분만 해당 spread되는 객체로 갱신된다.
    let user_extends = User {
        username: String::from("anderson_extends"),
        ..user1
    };

    println!("userextends is, age should be 12 {:?}", user_extends);

    fn build_user<'a>(email: &'a str, username: &'a str) -> UserByBuilder<'a> {
        UserByBuilder {
            username,
            email,
            age: 22,
        }
    }

    let mut user2 = build_user("test2@mail.com", "anderson2");
    println!("user2 is {:?}", user2);

    let tuple_color = Color(0, 0, 1);
    println!("tuplecolor is {:?}", tuple_color.2);
}
