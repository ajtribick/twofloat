use hexf::hexf64;

#[test]
fn create_from_macro_test() {
    let tf = twofloat_macro::twofloat!("0.3");
    assert_eq!(tf.hi(), hexf64!("0x1.3333333333333p-2"));
    assert_eq!(tf.lo(), hexf64!("0x1.999999999999ap-57"));
}
