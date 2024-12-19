use crate::{likely, unlikely};
use std::collections::HashMap;

trait IsUrlEncodeFriendly: Copy {
    fn is_urlencode_friendly(self) -> bool;
}
impl IsUrlEncodeFriendly for char {
    fn is_urlencode_friendly(self) -> bool {
        self.is_ascii_alphanumeric() || "-_.!~*'()".contains(self)
    }
}
impl IsUrlEncodeFriendly for u8 {
    fn is_urlencode_friendly(self) -> bool {
        self.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&self)
    }
}

pub fn len(input: &str) -> usize {
    input
        .chars()
        .map(|c| {
            if c.is_urlencode_friendly() {
                1
            } else {
                c.len_utf8() * 3
            }
        })
        .sum()
}
pub fn encode_inplace(input: &str, output: &mut String) {
    let output_len = len(input);
    output.reserve(output_len);
    for c in input.chars() {
        if c.is_urlencode_friendly() {
            output.push(c);
            continue;
        }
        let mut bytes = [0; 4];
        let len = c.encode_utf8(&mut bytes).len();
        bytes.into_iter().take(len).for_each(|c| {
            output.push('%');
            unsafe {
                output.push(char::from_digit((c >> 4) as u32, 16).unwrap_unchecked());
                output.push(char::from_digit((c & 0xf) as u32, 16).unwrap_unchecked());
            }
        });
    }
}
pub fn encode(input: &str) -> String {
    let mut output = String::new();
    encode_inplace(input, &mut output);
    output
}

pub trait IntoUrlEncoded {
    fn into_url_encoded(&self) -> String;
}

impl<S1: AsRef<str>, S2: AsRef<str>> IntoUrlEncoded for HashMap<S1, S2> {
    fn into_url_encoded(&self) -> String {
        let output_len: usize = self.keys().map(|s| s.as_ref().len()).sum::<usize>()
            + self.values().map(|v| self::len(v.as_ref())).sum::<usize>()
            + (self.len());
        let mut output = String::with_capacity(output_len);
        for (k, v) in self.iter() {
            output.push_str(k.as_ref()); // we admit that keys are url friendly
            output.push('=');
            self::encode_inplace(v.as_ref(), &mut output);
            output.push('&');
        }
        output.pop(); // pop last '&'
        output
    }
}

pub fn decode(input: &str) -> String {
    let mut it = input.chars();
    let mut output = Vec::with_capacity(input.len());
    loop {
        let c = match it.next() {
            Some(c) => c,
            None => return String::from_utf8(output).expect("invalide string in url parameters"),
        };
        if likely(c != '%') {
            assert!(c.is_ascii(), "url only expect ascii characters");
            output.push(c as u8);
            continue;
        }
        let c = it.next().expect("expect a char here");
        if unlikely(c == '%') {
            output.push(c as u8);
            continue;
        }
        let n = (c.to_digit(16).expect("expect two hex number after a %") << 4)
            + it.next()
                .expect("expect two hex number after a %")
                .to_digit(16)
                .expect("expect two hex number after a %");
        output.push(n as u8);
    }
}
pub fn decode_url_parameters(mut url: &str) -> Option<HashMap<&str, String>> {
    let mut cursor = url.find('?')? + 1;
    let mut result = HashMap::new();
    // let mut url = &url[cursor..];
    while let Some(end_index) = url.find('&') {
        let param = &url[cursor..end_index];
        let eq = param.find('=').expect("param without equal");
        result.insert(&param[0..eq], decode(&param[(eq + 1)..]));
        cursor = end_index + 1;
        url = &url[cursor..];
    }
    if let Some(eq) = url.find('=') {
        result.insert(&url[0..eq], url[(eq + 1)..].to_string());
    }
    Some(result)
}
