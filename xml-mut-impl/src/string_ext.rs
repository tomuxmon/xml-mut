#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeQuote {
    Double,
    Single,
}

impl AttributeQuote {
    pub fn as_char(&self) -> char {
        match self {
            AttributeQuote::Double => '"',
            AttributeQuote::Single => '\'',
        }
    }
}

pub trait StringExt {
    fn xml_escape_attribute_value(&self, quote: AttributeQuote) -> Self;
    fn xml_escape_node_text(&self) -> Self;
}

impl StringExt for String {
    fn xml_escape_attribute_value(&self, quote: AttributeQuote) -> Self {
        let mut escaped_value = String::with_capacity(self.len() + 2);
        let q = quote.as_char();
        escaped_value.push(q);
        for c in self.chars() {
            escaped_value.push_str(&escape_attribute_char(c, &quote));
        }
        escaped_value.push(q);
        escaped_value
    }

    fn xml_escape_node_text(&self) -> Self {
        let mut escaped_value = String::with_capacity(self.len());
        for c in self.chars() {
            escaped_value.push_str(&escape_text_char(c));
        }
        escaped_value
    }
}

fn escape_text_char(c: char) -> String {
    match c {
        '<' => "&lt;".to_string(),
        '&' => "&amp;".to_string(),
        _ => c.to_string(),
    }
}

fn escape_attribute_char(c: char, qoute: &AttributeQuote) -> String {
    match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '"' if *qoute == AttributeQuote::Double => "&quot;".to_string(),
        '\'' if *qoute == AttributeQuote::Single => "&apos;".to_string(),
        '&' => "&amp;".to_string(),
        '\n' => "&#xA;".to_string(),
        '\r' => "&#xD;".to_string(),
        _ => c.to_string(),
    }
}
