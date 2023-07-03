fn main() {
    let str = "kjsdfaadcaadaddc";
    let pat = "aadadd";

    let idx = kmp(str, pat).unwrap();

    println!("{}", idx);

    assert_eq!(idx, 9);
}

fn kmp(str: &str, pat: &str) -> Option<usize> {
    let next = build_next(pat);

    let mut i = 0;
    let mut j = 0;

    while i < str.len() {
        if str.chars().nth(i) == pat.chars().nth(j) {
            i += 1;
            j += 1;
        } else if j > 0 {
            j = next[j - 1];
        } else {
            i += 1;
        }

        if j == pat.len() {
            return Some(i - j);
        }
    }

    None
}

fn build_next(pat: &str) -> Vec<usize> {
    let mut next = vec![0; pat.len()];
    let mut prefix_len = 0;
    let mut i = 1;

    while i < pat.len() {
        if pat.chars().nth(prefix_len) == pat.chars().nth(i) {
            prefix_len += 1;
            next.push(prefix_len);
            i += 1;
        } else if prefix_len == 0 {
            next.push(0);
            i += 1
        } else {
            prefix_len = next[prefix_len - 1];
        }
    }
    next
}
