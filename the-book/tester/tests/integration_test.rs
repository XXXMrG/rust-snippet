use tester;

mod common;

// 集成测试只针对于库 crate，而最终作为入口的二进制 create 则不需要集成测试
// lib 的设计需要考虑接口暴露的正确性，而作为直接执行的 main.rs，逻辑简单，则无需集成测试


// 单元测试无需考虑的私有性问题，在集成测试时则会暴露出来
#[test]
fn test_area() {
    let r = tester::Rectangle { width: 4, height: 4};

    assert_eq!(r.get_area(), 16);
}