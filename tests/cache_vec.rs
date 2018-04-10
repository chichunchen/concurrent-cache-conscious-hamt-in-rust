extern crate cchamt;

use cchamt::{CCVec};

#[test]
fn test_new_vec() {
    let mut a: CCVec<i32> = CCVec::new();
}

#[test]
fn test_insert_vec() {
    let mut a: CCVec<i32> = CCVec::new();
    a.insert(0, 1);
    a.insert(0, 1);
    a.insert(0, 1);
    a.insert(0, 1);
    a.insert(0, 1);
    a.insert(0, 1);
}
