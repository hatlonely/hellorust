#[cfg(test)]
mod tests {
    // use std::collections::BTreeMap;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::LinkedList;

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
    fn string() {
        {
            let a = "hello";
            let b = "world";
            let c = [a, b].join(" ");
            let d = format!("{} {}", a, b);
            assert_eq!(c, "hello world");
            assert_eq!(d, "hello world");
        }
        {
            let mut a = "hello".to_owned();
            a.push_str(" world");
            assert_eq!(a, "hello world");
            a += " world";
            assert_eq!(a, "hello world world");
        }
        {
            let mut a = "hello".to_owned();
            let b = " world".to_owned();
            a.push_str(&b);
            assert_eq!(a, "hello world");
            a += &b;
            assert_eq!(a, "hello world world");
            let c = a + &b;
            assert_eq!(c, "hello world world world");
        }
        {
            let a = "hello world".to_owned();
            let v: Vec<&str> = a.split(" ").collect();
            assert_eq!(v, vec!["hello", "world"]);
        }
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
        assert_eq!(vi.iter().position(|&r| r == 66).unwrap(), 6);
    }

    #[test]
    fn list() {
        let mut li: LinkedList<i64> = LinkedList::new();
        li.push_back(1);
        assert_eq!(li.pop_back(), Some(1));
        assert_eq!(li.pop_back(), None);
        assert!(li.is_empty());
    }

    #[test]
    fn binaryheap() {
        let mut bh: BinaryHeap<i64> = BinaryHeap::new();
        bh.push(3);
        bh.push(5);
        bh.push(2);
        bh.push(4);
        bh.push(1);
        assert_eq!(bh.peek(), Some(&5));
        assert_eq!(bh.pop(), Some(5));
        assert_eq!(bh.pop(), Some(4));
    }

    #[test]
    fn hashmap() {
        // let mut hm: HashMap<&str, i64> = HashMap::new();
        // let mut tm: BTreeMap<&str, i64> = [("four", 4), ("five", 5)].iter().cloned().collect();
        let mut hm: HashMap<&str, i64> = [("four", 4), ("five", 5)].iter().cloned().collect();
        hm.insert("one", 1);
        hm.insert("two", 2);
        assert_eq!(hm["one"], 1);
        hm.entry("three").or_insert(6);
        hm.entry("three").or_insert(3);
        assert_eq!(hm["three"], 6);
        hm.insert("three", 3);
        assert_eq!(hm["three"], 3);
        for (k, v) in &hm {
            println!("{} => {}", k, v);
        }
        assert!(hm.contains_key("one"));
        hm.remove("one");
        assert!(!hm.contains_key("one"));
        assert!(!hm.is_empty());
        for k in hm.keys() {
            println!("{} => {}", k, hm[k]);
        }
    }
}
