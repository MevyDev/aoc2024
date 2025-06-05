use std::collections::HashMap;
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

    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut res: i32 = 0;

    for num in nums2 {
        *map.entry(num).or_insert(0) += 1;
    }

    for i in 0..nums1.len() {
        let cur: i32 = nums1[i];
        res += cur * map.get(&cur).unwrap_or(&0);
    }

    println!("{}", res);

    Ok(())
}
