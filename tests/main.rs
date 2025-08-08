use ops_derive2::AutoAll;

#[test]
fn tuple_vector2() {
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Vector2(i32, i32);

    let a = Vector2(5, 5);
    let b = Vector2(6, 2);
    assert_eq!(a + b, Vector2(11, 7));
}

#[test]
fn tuple_vector3() {
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Vector3(i32, i32, i32);

    let a = Vector3(5, 5, 2);
    let b = Vector3(6, 2, 8);
    assert_eq!(a + b, Vector3(11, 7, 10));
}

#[test]
fn struct_vector2() {
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Vector2 {
        x: i32,
        y: i32,
    }

    let a = Vector2 { x: 5, y: 5 };
    let b = Vector2 { x: 6, y: 2 };
    assert_eq!(a + b, Vector2 { x: 11, y: 7 });
}

#[test]
fn struct_vector3() {
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Vector3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let a = Vector3 { x: 5, y: 5, z: 2 };
    let b = Vector3 { x: 6, y: 2, z: 8 };
    assert_eq!(a + b, Vector3 { x: 11, y: 7, z: 10 });
}

#[test]
fn tuple_double() {
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Position(i32, i32);
    #[derive(Debug, PartialEq, Eq, AutoAll)]
    struct Vector2(i32, i32);
    impl From<Position> for Vector2 {
        fn from(value: Position) -> Self {
            Self(value.0, value.1)
        }
    }
    impl From<Vector2> for Position {
        fn from(value: Vector2) -> Self {
            Self(value.0, value.1)
        }
    }

    let origin = Position(0, 0);
    let right = Vector2(1, 0);
    assert_eq!(origin + right, Position(1, 0));
}
