use time::*;

fn build_ok() {
    let b1: &[u8] = &[0_u8, 1];
    assert_eq!(b1, &[0]);
}

fn build_failure() {
    let b1: &[u8] = &[0_u8, 1];
    assert_eq!(b1, &[]);
}
