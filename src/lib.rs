#[cfg(test)]
mod tests {
    #[test]
    fn format() {
        println!("{}", 123.456789);
        println!("{:.3}", 123.456789);
        println!("hex: 0x{:08x}", 123);
        println!("oct: 0o{:016o}", 123);
        println!("bin: 0b{:064b}", 123);
    }

    #[test]
    fn primitive_type() {
        let tup = (1, 1.2, "hello", true);
        let (i, f, s, b) = tup;
        println!("{}, {}, {}, {}", i, f, s, b);
    }

    #[test]
    fn slice() {
        let langs = ["rust", "golang", "c++", "python"];
        println!("{:?}, {}, {:?}", langs, langs[0], &langs[1..3]);
    }

    #[test]
    fn vector() {
        let mut vi: Vec<i64> = vec![0, 1, 2, 3, 4, 5];
        for i in 6..=9 {
            vi.push(i);
        }
        assert_eq!(vi, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        vi.pop();
        assert_eq!(vi, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(vi[5], 5);
        vi[6] = 66;
        assert_eq!(vi, vec![0, 1, 2, 3, 4, 5, 66, 7, 8]);
    }
}
