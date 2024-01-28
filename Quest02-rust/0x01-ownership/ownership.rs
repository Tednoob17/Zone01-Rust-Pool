
pub fn first_subword(s: String) -> String {
    let mut iter = s.chars().peekable();
    let mut subword = String::new();

    while let Some(&c) = iter.peek() {
        if c.is_alphanumeric() {
            subword.push(c);
            iter.next();
        } else if c == '_' || c.is_uppercase() {
            break;
        } else {
            iter.next();
        }
    }

    if subword.is_empty() {
        None
    } else {
        Some(subword)
    }
}