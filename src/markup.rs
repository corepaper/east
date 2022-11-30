/// Escape HTML string to output.
pub fn escape_to_string(input: &str, output: &mut String) {
    for b in input.bytes() {
        match b {
            b'&' => output.push_str("&amp;"),
            b'<' => output.push_str("&lt;"),
            b'>' => output.push_str("&gt;"),
            b'"' => output.push_str("&quot;"),
            _ => unsafe { output.as_mut_vec().push(b) },
        }
    }
}

/// Escape HTML string.
pub fn escape(input: &str) -> String {
    let mut s = String::new();
    escape_to_string(input, &mut s);
    s
}

/// A pre-escaped string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PreEscaped<T: AsRef<str>>(pub T);

/// A pre-escaped owned string.
pub type Markup = PreEscaped<String>;
