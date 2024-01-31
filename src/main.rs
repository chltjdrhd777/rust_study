mod chapter1; // mod는 해당 스크립트 파일을 모듈로서 가져올 수 있게 해준다. (내부 pub 정의된 애들이 export처럼 들고올 수 있음.)
mod chapter4;
mod chapter5;

fn main() {
    chapter5::chapter5();
}
