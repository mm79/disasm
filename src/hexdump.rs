pub fn dump(buf: &[u8]) {
    let bytes_per_line = 0x10;

    for (i, chunk) in buf.chunks(bytes_per_line).enumerate() {
        print!("{:08x}: ", i * bytes_per_line);

        let mut i = 0;
        for byte in chunk.iter() {
            print!("{:02x} ", byte);
            i += 1;

            if i % 8 == 0 {
                print!(" ");
            }
        }

        let nchunks = chunk.len();
        let nleft = bytes_per_line - nchunks;

        for i in 0..nleft {
            print!("   ");

            if (nchunks + i) % 8 == 0 {
                print!(" ");
            }

            if i == nleft - 1 {
                print!(" ");
            }
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
