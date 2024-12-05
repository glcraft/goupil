use std::collections::HashMap;

pub fn len(input: &str) -> usize {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
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
        if c.is_ascii_alphanumeric() {
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
