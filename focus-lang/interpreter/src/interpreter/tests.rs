use crate::{object::Value, Interpreter};

#[test]
fn add() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
        let a b: (int -> int) = 2 + b
        a 3
        "#,
        )
        .expect("Unable to interpret.");
    assert_eq!(value, Value::Integer(5));
}

#[test]
fn assign() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
        type Point = {x: int, y: int}
        let a = Point {x: 1, y: 2}
        a = 2
        "#,
        )
        .expect("Unable to interpret.");
    assert_eq!(value, Value::Integer(5));
}

#[test]
fn function_w_closure_arg() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
        let a f: ((int -> int) -> int) = f 1
        a (fn n -> n + 3)
        "#,
        )
        .expect("Unable to interpret function.");
    assert_eq!(value, Value::Integer(4))
}

#[test]
fn closure_w_captured_local() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
        let a: (() -> (() -> int)) = 
            let index = 0
            fn -> 
                index = index + 1
                index
        let f = a ()
        f ()
        f ()
        "#,
        )
        .expect("Unable to interpret.");
    println!("{value:?}");
    assert_eq!(value.deref_value(), Value::Integer(2));
}

#[test]
fn complex_type_path() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
        type Point = {x:int,y:int}
        type Line = {a:Point,b:Point}
        let a = Point {x:1,y:2}
        let b = Point {x:3,y:4}
        let line = Line {a:a,b:b}
        line.a.x
        "#,
        )
        .expect("Unable to interpret.");
    println!("{value:?}");
    assert_eq!(value.deref_value(), Value::Integer(1));
}

#[test]
fn array() {
    let mut interpreter = Interpreter::new();
    let value = interpreter
        .interpret_str(
            r#"
    let a = [1, 2]
    a[1]
    "#,
        )
        .expect("Unable to interpret.");
    assert_eq!(value, Value::Integer(2));
}
