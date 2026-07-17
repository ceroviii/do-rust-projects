use crate::parser::{self, LineType};

pub fn render(content: String, file_name: String) -> String {
    let body = String::new();

    // render
    //parser::classify_line()

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
