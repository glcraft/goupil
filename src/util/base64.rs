///! Base64 implementation
///! https://fr.wikipedia.org/wiki/Base64

#[inline]
pub fn url_encode(buffer: &[u8]) -> String {
    const BASE64URL: &[std::ascii::Char] =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
            .as_ascii()
            .unwrap();
    impl_encode(BASE64URL, buffer)
}
#[inline]
pub fn encode(buffer: &[u8]) -> String {
    const BASE64URL: &[std::ascii::Char] =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .as_ascii()
            .unwrap();
    impl_encode(BASE64URL, buffer)
}
fn impl_encode(base: &[std::ascii::Char], buffer: &[u8]) -> String {
    let mut result = String::with_capacity(buffer.len() * 4 / 3);
    let mut it = buffer.iter();
    for _ in 0..(buffer.len() / 3) {
        let data = unsafe {
            (
                it.next().unwrap_unchecked(),
                it.next().unwrap_unchecked(),
                it.next().unwrap_unchecked(),
            )
        };
        let b64data = [
            base[((data.0 & 0xfc) >> 2) as usize],
            base[(((data.0 & 0x03) << 4) + ((data.1 & 0xf0) >> 4)) as usize],
            base[(((data.1 & 0x0f) << 2) + ((data.2 & 0xc0) >> 6)) as usize],
            base[(data.2 & 0x3f) as usize],
        ];
        result.push_str(b64data.as_str());
    }
    let data = (it.next(), it.next(), it.next());
    if let Some(b1) = data.0 {
        let index = (b1 & 0xfc) >> 2;
        result.push(base[index as usize].into());
        if let Some(b2) = data.1 {
            let index = ((b1 & 0x03) << 4) + ((b2 & 0xf0) >> 4);
            result.push(base[index as usize].into());

            if let Some(b3) = data.2 {
                let index = ((b2 & 0x0f) << 2) + ((b3 & 0xc0) >> 6);
                result.push(base[index as usize].into());
                let index = b3 & 0x3f;
                result.push(base[index as usize].into());
            } else {
                let index = (b2 & 0x0f) << 2;
                result.push(base[index as usize].into());
                result.push('=');
            }
        } else {
            let index = (b1 & 0x03) << 4;
            result.push(base[index as usize].into());
            result.push('=');
            result.push('=');
        }
    }
    return result;
}
