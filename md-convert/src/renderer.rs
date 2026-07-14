use crate::parser::LineType;

pub fn render(lines: String, file_name: String) -> String {
    let body = String::new();

    // render

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <title>{}</title>
</head>
<body>
  {}
</body>
</html>"#,
        file_name, body
    )
}
