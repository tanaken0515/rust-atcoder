use proconio::input;

fn main() {
    input! {
        n: u32,
        y: u32,
    }

    let mut honest = false;
    let max10000: u32 = y / 10000;

    'outer: for a in 0..=max10000 {
        let max5000 = (y - 10000 * a) / 5000;

        for b in 0..=max5000 {
            let c = (n - (a + b)) as i32;
            if c < 0 {
                break
            }

            if 10000 * a + 5000 * b + 1000 * (c as u32) == y {
                println!("{} {} {}", a, b, c);
                honest = true;
                break 'outer
            }
        }
    }

    if !honest {
        println!("-1 -1 -1");
    }
}
