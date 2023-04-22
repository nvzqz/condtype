use condtype::*;

#[test]
fn condval_true() {
    let x = condval!(if true { "a" } else { 1 });
    assert_eq!(x, "a");
}

#[test]
fn condval_false() {
    let x = condval!(if false { "a" } else { 1 });
    assert_eq!(x, 1);
}

#[test]
fn condval_const() {
    pub const COND: bool = false;

    let x = condval!(if COND { "a" } else { 1 });
    assert_eq!(x, 1);
}

#[test]
fn condval_path() {
    mod cond {
        pub const COND: bool = false;
    }

    let x = condval!(if cond::COND { "a" } else { 1 });
    assert_eq!(x, 1);
}

#[test]
fn condval_not_true() {
    let x = condval!(if { !true } { "a" } else { 1 });
    assert_eq!(x, 1);
}

#[test]
fn condval_and() {
    let x = condval!(if { true && false } { "a" } else { 1 });
    assert_eq!(x, 1);
}

#[test]
fn condval_paren() {
    let x = condval!(if (true && false) { "a" } else { 1 });
    assert_eq!(x, 1);
}
