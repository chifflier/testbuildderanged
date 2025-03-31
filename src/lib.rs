use time::*;

fn build_failure() {
    let b1: &[u8] = &[0_u8, 1];
    assert_eq!(b1, &[]);
}
