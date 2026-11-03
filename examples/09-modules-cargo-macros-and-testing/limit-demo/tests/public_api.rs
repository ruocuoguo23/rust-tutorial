use limit_demo::parse_limit;

#[test]
fn public_api_accepts_boundaries() {
    assert_eq!(parse_limit("0").unwrap(), 0);
    assert_eq!(parse_limit("65535").unwrap(), 65535);
}
