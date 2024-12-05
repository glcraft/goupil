pub mod base64;
pub mod sha;

pub fn urlencode(input: &str) -> String {
    let len = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                1
            } else {
                c.len_utf8() * 3
            }
        })
        .sum();
    let mut output = String::with_capacity(len);
    // let mut urlencoded = String::new();
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            output.push(c);
        } else {
            let (bytes, len) = {
                let mut bytes = [0; 4];
                let len = { c.encode_utf8(&mut bytes).len() };
                (bytes, len)
            };
            // urlencoded.clear();
            // urlencoded.reserve(s.len() * 3);
            bytes.into_iter().take(len).for_each(|c| {
                output.push('%');
                unsafe {
                    output.push(char::from_digit((c >> 4) as u32, 16).unwrap_unchecked());
                    output.push(char::from_digit((c & 0xf) as u32, 16).unwrap_unchecked());
                }
            });
        }
    }
    output
}
