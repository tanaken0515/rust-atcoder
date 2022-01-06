use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    // println!("{}", a.iter().reduce(|accum, item| &(accum | item)).unwrap().trailing_zeros());
    println!("{}", a.iter().map(|item| item.trailing_zeros()).min().unwrap());
}
