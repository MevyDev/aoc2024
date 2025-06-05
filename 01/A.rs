use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nums1 = Vec::new();
    let mut nums2 = Vec::new();

    for cur_line in reader.lines() {
        let line = cur_line?;

        let mut parts = line.split_whitespace();

        let num1: i32 = parts.next().unwrap().parse().unwrap();
        let num2: i32 = parts.next().unwrap().parse().unwrap();

        nums1.push(num1);
        nums2.push(num2);
    }

    nums1.sort();
    nums2.sort();

    let mut res: i32 = 0;

    for i in 0..nums1.len() {
        let dif = nums1[i] - nums2[i];
        let abs_dif = dif.abs();

        res += abs_dif;
    }

    println!("{}", res);

    Ok(())
}
