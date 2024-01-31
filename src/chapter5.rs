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

pub fn chapter5_2() {
    // 튜플 구조체 : 자바스크립트에서 인자를 하나하나 정하기보다, 객체로 한번에 전달하는 것이 편한 것처럼
    // rust에서는 튜플 형태로 parameter을 구성하는 것이 가능하다.
    let dimensions = (50, 30);

    let area = get_area(dimensions);

    println!("area is {}", area);

    fn get_area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    //보통은 parameter을 정의할 경우 구조체를 사용하는 것이 훨씬 직관적이다.
    //해당 방식은 타입스크립트의 인터페이스와 매우 흡사하다.
    //튜플때와 다르게, 명확하게 dimensions의 어떤 property를 사용하려는 것인지 나타내는 것을 보길 바란다.

    //중요! parameter을 객체 구조체 형태로 정의할 때에는, 습관적으로 "&" 를 이용해서 소유권을 빌리길 권장한다.
    //자바스크립트때와 다르게, 그냥 소유권을 넘겨버릴 경우 해당 객체를 다시 사용할 수 없기 때문이다.(함수 리턴으로 다시 내보내는게 아닌 이상)
    //참고로, 러스트는 구조체로 생성된 인스턴스를 그대로 콘솔을 찍으려면 에러를 낸다
    //따라서 struct에 debug trait 어노테이션을 추가해줘야하며, 콘솔의 부분에는 ":?" 를 사용줘야 한다.
    //중요한 것은 해당 작업은 어디까지나 구조체로 초기화된 인스턴스를 그대로 콘솔을 찍으려 할 때에 발생하는 문제다.
    //그것이 아래에 콘솔을 찍을 때 :? 없이 area_2를 콘솔 찍을 수 있었던 이유다.

    //그리고, 사소하지만 :#? 를 이용해서 예쁘게 프린트가 되는것도 가능하니 인스턴스를 콘솔 찍으려면 되도록이면 이것을 이용하자.
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let dimensions_2 = Rectangle {
        length: 50,
        width: 30,
    };

    let area_2 = get_area_2(&dimensions_2);

    println!("dimensions_2 is {:#?}", dimensions_2);
    println!("area_2 is {}", area_2);

    fn get_area_2(dimensions: &Rectangle) -> u32 {
        dimensions.length * dimensions.width
    }
}

pub fn chater5_3() {
    // 메소드 구현에 대해서
    // 러스트는 구현체(impl)에 메소드를 정의할 때 구현체를 따로 작성하는 방식으로 만든다
    // (특이하게도, 자바스크립트의 구현체와는 다르게 여기는 확장의 개념에 가깝다.)

    // 메소드의 인자로 처음 오는 것은 무조건 self(javascript로 따지면 this 이다.)
    // 클래스에서 메소드 구현할 때의 형태를 기억해보자.
    // 주의할 점은, self에 대해서 소유권을 빌리는 형태로 가져갈 경우 만들어질 인스턴스에 대한 read-only이며
    // &mut 을 사용해서 self를 참조할 경우, 해당 메소드 호출결과가 객체의 변경까지 이어질 수 있음을 이해해야 한다.

    // 또한 인스턴스 내의 메서드를 호출할 경우, 러스트는 스마트하게 "자동 참조" 를 하여 호출을 진행한다.
    // 예를 들어, p1.distance(&p2); 라고 한다면
    // 사실 이것은 자동적으로 p1의 소유권을 빌린 뒤, 메서드를 호출하는 것과 같다.
    // 즉, (&p1).distance(&p2) 와 동일하다.

    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }
    impl Rectangle {
        fn area(&mut self) -> u32 {
            self.width * self.length
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
        fn associated_function(size: u32) -> Rectangle {
            Rectangle {
                length: size,
                width: size,
            }
        }
    }

    let rect1 = Rectangle {
        length: 50,
        width: 40,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 30,
    };

    // 아래에서 볼 수 있는 것처럼, self는 호출 당시에 args의 자리에 차지하지 않는다.
    // 인자로 들어가면 자동으로 2번째에 들어가게 되는 것이다.
    println!("Check the holding {}", rect1.can_hold(&rect2));

    // impl에 구현되는 함수 가운데 &self를 인자로 받지 않는 함수를 연관함수라고 부른다.
    // 해당 연관함수는 보통 구조체의 인스턴스를 반환하는 생성자로 자주 사용된다
    // 메소드가 아니라 함수이고, &self를 받지 않기 때문에 해당 구조체와 함께 동작할 인스턴스를 참조하지 않는다.
    // 따라서 사용을 할 때에도 :: 문법을 사용해야 한다.
    // 이는 자바스크립트의 static 메소드와 매우 유사하다.

    let rect3 = Rectangle::associated_function(100);
}
