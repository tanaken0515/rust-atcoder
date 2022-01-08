use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut ans: u32 = 0;
    let available500: u32 = if x / 500 > a { a } else { x / 500 };
    let mut available100: u32;

    for n in 0..=available500 {
        available100 = if (x - 500 * n) / 100 > b { b } else { (x - 500 * n) / 100 };

        for m in 0..=available100 {
            if x - 500 * n - 100 * m <= 50 * c {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
