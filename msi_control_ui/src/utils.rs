use ksni::Icon;
use std::io::Cursor;

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn icon_from_bytes(bytes: &[u8]) -> Icon {
    let icon_cursor = Cursor::new(bytes);
    let icon_decoder = png::Decoder::new(icon_cursor);
    let mut icon_reader = icon_decoder.read_info().unwrap();
    let mut icon_buf = vec![0; icon_reader.info().raw_bytes()];
    icon_reader.next_frame(&mut icon_buf).unwrap();
    Icon {
        data: icon_buf,
        height: 32,
        width: 32,
    }
}
