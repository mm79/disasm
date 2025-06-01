pub fn dump(buf: &[u8]) {
    let bytes_per_line = 0xf;

    for (i, chunk) in buf.chunks(bytes_per_line).enumerate() {
        print!("{:08x}: ", i * bytes_per_line);

        for byte in chunk.iter() {
            print!("{:02x} ", byte);
        }

        for _ in 0..(bytes_per_line - chunk.len()) {
            print!("   ");
        }

        print!("|");

        for byte in chunk {
            let c = *byte as char;
            if c.is_ascii_graphic() || c == ' ' {
                print!("{}", c);
            } else {
                print!(".");
            }
        }

        println!("|");
    }
}
