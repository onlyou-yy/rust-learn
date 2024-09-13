use crate::garden::vegetables::asparagus::add_two;

mod garden;

fn add_num(a:i32,b:i32) -> i32{
  a + b
}


#[test]
fn test_add_num(){
    assert_eq!(add_num(1, 2),3);
}

#[test]
fn test_add_two(){
  assert_eq!(add_two(1),3);
}
