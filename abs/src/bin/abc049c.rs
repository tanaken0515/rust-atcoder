use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut reversed_s = s.chars().rev().collect::<String>();
    // let words = ["dream", "dreamer", "erase", "eraser"].map(|word| word.chars().rev().collect::<String>());
    let words = ["maerd", "remaerd", "esare", "resare"];

    let mut yes = true;
    while reversed_s != "" {
        let mut splitable = false;
        for word in words.iter() {
            if word.len() > reversed_s.len() {
                continue
            }

            let remain = reversed_s.split_off(word.len());
            if &&reversed_s == word {
                splitable = true;
                reversed_s = remain;
                break
            } else {
                reversed_s = reversed_s + &remain;
            }
        }

        if !splitable {
            yes = false;
            break
        }
    }
    println!("{}", if yes { "YES" } else { "NO" });
}
