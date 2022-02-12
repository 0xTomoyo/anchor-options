pub fn read_pyth_product_attribute<'d>(data: &'d [u8], attribute: &[u8]) -> Option<&'d [u8]> {
    let mut idx = 0;

    while idx < data.len() {
        let key_len = data[idx] as usize;
        idx += 1;

        if key_len == 0 {
            continue;
        }

        let key = &data[idx..idx + key_len];
        idx += key_len;

        let val_len = data[idx] as usize;
        idx += 1;

        let value = &data[idx..idx + val_len];
        idx += val_len;

        if key == attribute {
            return Some(value);
        }
    }

    None
}
