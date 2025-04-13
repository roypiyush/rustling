use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;

#[inline(always)]
fn parse_int(bytes: &[u8]) -> i32 {
    let mut value = 0i32;
    let mut negative = false;
    let mut i = 0;

    if bytes.get(0) == Some(&b'-') {
        negative = true;
        i += 1;
    }

    while i < bytes.len() {
        let b = bytes[i];
        value = value * 10 + (b - b'0') as i32;
        i += 1;
    }

    if negative {
        -value
    } else {
        value
    }
}

fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    let file = File::open("/mnt/e/large_file.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let data = &mmap[..];

    let mut line_starts = vec![0usize];
    for (i, &b) in data.iter().enumerate() {
        if b == b'\n' {
            line_starts.push(i + 1);
        }
    }

    line_starts
        .par_windows(2)
        .enumerate()
        .for_each(|(idx, window)| {
            let line = &data[window[0]..window[1]];
            let mut values = [0i32; 25];
            let mut col = 0;
            let mut start = 0;

            for i in 0..line.len() {
                if line[i] == b',' || line[i] == b'\n' {
                    values[col] = parse_int(&line[start..i]);
                    col += 1;
                    start = i + 1;
                }
            }

            // last value (if line does not end with '\n' or ',')
            if col < 25 && start < line.len() {
                values[col] = parse_int(&line[start..]);
            }

            if idx < 5 {
                println!("Line {}: {:?}", idx + 1, values);
            }
        });

    println!("Done in {:?}", start.elapsed());
    Ok(())
}
