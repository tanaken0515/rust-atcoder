use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut ans: u32 = 0;
    for x in 1..=n {
        let s: u32 = x.to_string().chars().map(|c| c as u32 - 48).sum();
        if s >= a && s <= b {
            ans += x;
        }
    }

    println!("{}", ans);
}
