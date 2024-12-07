pub mod base64;
pub mod sha;
pub mod urlencode;

#[macro_export]
macro_rules! hashmap {
    ( $(($key:expr, $value:expr $(,)?)),+ $(,)? ) => {{
        let mut hm = std::collections::HashMap::new();
        $(
        hm.insert($key, $value);
        )+
        hm
    }}
}

#[inline]
#[cold]
fn cold() {}

#[inline]
pub fn likely(b: bool) -> bool {
    if !b {
        cold()
    }
    b
}

#[inline]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold()
    }
    b
}
