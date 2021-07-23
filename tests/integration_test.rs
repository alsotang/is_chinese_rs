use is_chinese;

#[test]
fn is_all_chinese() {
    assert!(is_chinese::is_chinese("中国"));
    assert!(is_chinese::is_chinese("你要吃什么吗？！"));
    assert!(is_chinese::is_chinese("扁担宽，板凳长，扁担想绑在板凳上。"));
}

#[test]
fn is_all_not_chinese() {
    assert!(!is_chinese::is_chinese("中国ss"));
    assert!(!is_chinese::is_chinese("ss"));
    assert!(!is_chinese::is_chinese("中国?"));
    assert!(!is_chinese::is_chinese(
        "ss扁担宽，板凳长，扁担想绑在板凳上。"
    ));
    assert!(!is_chinese::is_chinese(
        "扁担宽，板凳长，扁担想绑在板凳上。ss"
    ));
}
