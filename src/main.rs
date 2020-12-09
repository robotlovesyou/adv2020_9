fn load_input<'a>(lines: impl Iterator<Item=&'a str>) -> Vec<i64> {
    lines.map(|line| line.parse::<i64>().expect("invalid line")).collect()
}

fn is_valid(n: i64, prev: &[i64]) -> bool {
    for v in prev.iter() {
        if *v >= n {
            continue;
        }
        let rem = n - *v;
        if rem == *v {
            continue;
        }
        if prev.contains(&rem) {
            return true
        }
    }
    false
}

fn find_missing(list: &[i64]) -> i64 {
    for i in 25..list.len() {
        let n = list[i];
        if !is_valid(n, &list[i-25..i]) {
            return n;
        }
    }
    0
}

fn find_key(target: i64, list: &[i64]) -> i64 {
    for i in 0..list.len() {
        let n = list[i];
        if n >= target {
            continue
        }
        let mut total = n;
        let mut min_n = n;
        let mut max_n = n;
        for m in list[i+1..].iter() {
            total += m;
            min_n = i64::min(min_n, *m);
            max_n = i64::max(max_n, *m);
            if total == target {
                return min_n + max_n;
            }
            if total > target {
                break;
            }
        }
    }
    0
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    let list = load_input(lines);

    let missing = find_missing(&list);
    let key = find_key(missing, &list);


    println!("The missing number is {}", missing);
    println!("The key is {}", key);
}