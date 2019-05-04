mod parsing;
mod routing;

// Defines a few tests to show the basic testing utils

#[test]
fn test_1() {
    assert_eq!(22, 22);
    assert_ne!(22, 42);
}

#[test]
#[should_panic]
fn test_2() {
    "blub".parse::<i8>().unwrap();
}

#[test]
#[should_panic(expected = "Error message")]
fn test_3() {
    "blub".parse::<i8>().expect("Error message");
}

#[test]
#[ignore]
fn test_4() {
    assert_eq!(1, 2);
    assert!(false);
}