pub const fn str_to_array<const N: usize>(s: &str) -> Option<[u8; N]> {
    let bytes = s.as_bytes();
    if bytes.len() > N {
        return None;
    }

    let mut data = [0; N];
    let mut i = 0;
    while i < bytes.len() && bytes[i] > 0 {
        data[i] = bytes[i];
        i += 1;
    }

    Some(data)
}
