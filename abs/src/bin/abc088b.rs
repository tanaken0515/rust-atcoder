use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.sort();
    a.reverse();
    let sum = a.iter().enumerate().fold(0, |acc, (i, value)| {
        acc + if i % 2 == 0 { value * 1 } else { value * -1 }
    });
    println!("{}", sum);
}
