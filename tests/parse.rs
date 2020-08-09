use css_typegen::css_typegen;

// Test that the macro compiles
css_typegen!("css/styles.css", "css/other.css", "css/folder");

#[test]
fn test_field() {
    assert_eq!(C.kebab_case, "kebab-case");
    assert_eq!(C.snake_case, "snake_case");
}
