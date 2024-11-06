use proconio::input;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    input! {
        N:usize,
        K:usize
    };
    input! {
        S: String
    };

    let mut count:i64 = 0;
    let chers: Vec<char> = S.chars().collect();
    let perms = chers.iter().permutations(N);
    let mut set = HashSet::new();
    for perm in perms {
        let s = perm.iter().copied().collect::<String>();
        if set.contains(&s) {
            continue;
        } else {
            set.insert(s);
        }
        let mut res = false;
        for i in 0..=(N-K) {
            let mut res2 = true;
            for j in 0..K {
                if perm[i+j] != perm[i+K-1-j] {
                    res2 = false;
                    break;
                }
            }
            if res2 {
                res = true;
                break;
            }
        }
        if !res {
            count += 1;
        }
    }
    println!("{}", count);
}