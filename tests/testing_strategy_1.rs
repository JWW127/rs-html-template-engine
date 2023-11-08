use html_template_engine::*;

#[cfg(test)]
#[test] //checks if literal string stored in (s) is tokenized as ContentType::Literal
fn check_literal_test() {
    let s = "<h1>Hello World</h1>";
    assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s))
}

#[test] //check if content type is template variable type
fn check_template_var_test() {
    let content = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: "name".to_string(),
        tail: Some(" ,welcome".to_string()),
    };
    assert_eq!(
        ContentType::TemplateVariable(content),
        get_content_type("Hi {{name}} ,welcome")
    );
}

#[test] // check if content is a ForTag
fn check_for_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::ForTag),
        get_content_type(
            "{% for name in names %}
                ,welcome"
        )
    );
}

#[test] //check for success `if` tag tokenized
fn check_if_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::IfTag),
        get_content_type("{% if name == 'Bob' %}")
    );
}

#[test]
fn check_symbol_string_test() {
    assert_eq!(true, check_symbol_string("{{Hello}}", "{{"))
}

#[test]
fn check_symbol_pair_test() {
    assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"))
}

#[test]
fn check_get_expression_data_test() {
    let expression_data = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: "name".to_string(),
        tail: Some(" ,welcome".to_string()),
    };
    assert_eq!(expression_data, get_expression_data("Hi {{name}} ,welcome"));
}

#[test]
fn check_get_index_for_symbol_test() {
    assert_eq!((true, 3), get_index_for_symbol("Hi {name} ,welcome", '{'))
}
