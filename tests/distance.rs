use salt_level::distance_cm;

#[test]
fn distance_is_calculated() {
    assert!((distance_cm(58.31) - 1.0).abs() < 0.0001);
    assert!((distance_cm(116.62) - 2.0).abs() < 0.0001);
}