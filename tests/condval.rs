use condtype::*;

#[test]
fn condval_true() {
    let x = condval!(true, "a", 1);
    assert_eq!(x, "a");
}

#[test]
fn condval_false() {
    let x = condval!(false, "a", 1);
    assert_eq!(x, 1);
}
