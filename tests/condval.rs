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

#[test]
fn condval_else_if1() {
    let x = condval!(if true {
        "a"
    } else if false {
        1
    } else {
        42.0
    });
    assert_eq!(x, "a");
}

#[test]
fn condval_else_if2() {
    let x = condval!(if false {
        "a"
    } else if false {
        1
    } else {
        42.0
    });
    assert_eq!(x, 42.0);
}

#[test]
fn condval_else_if3() {
    let x = condval!(if false {
        "a"
    } else if false {
        1
    } else if true {
        42.0
    } else {
        [1, 2, 3]
    });
    assert_eq!(x, 42.0);
}

#[test]
fn condval_else_if4() {
    let x = condval!(if false {
        "a"
    } else if false {
        1
    } else if false {
        42.0
    } else {
        [1, 2, 3]
    });
    assert_eq!(x, [1, 2, 3]);
}
