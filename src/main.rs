use std::collections::HashSet;
use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

const HIRAGANA: [(&str, &str); 46] = [
    ("あ", "a"), ("い", "i"), ("う", "u"), ("え", "e"), ("お", "o"),
    ("か", "ka"), ("き", "ki"), ("く", "ku"), ("け", "ke"), ("こ", "ko"),
    ("さ", "sa"), ("し", "shi"), ("す", "su"), ("せ", "se"), ("そ", "so"),
    ("た", "ta"), ("ち", "chi"), ("つ", "tsu"), ("て", "te"), ("と", "to"),
    ("な", "na"), ("に", "ni"), ("ぬ", "nu"), ("ね", "ne"), ("の", "no"),
    ("は", "ha"), ("ひ", "hi"), ("ふ", "fu"), ("へ", "he"), ("ほ", "ho"),
    ("ま", "ma"), ("み", "mi"), ("む", "mu"), ("め", "me"), ("も", "mo"),
    ("や", "ya"), ("ゆ", "yu"), ("よ", "yo"),
    ("ら", "ra"), ("り", "ri"), ("る", "ru"), ("れ", "re"), ("ろ", "ro"),
    ("わ", "wa"), ("を", "wo"),
    ("ん", "n")
];

fn main() {
    println!("{}", GAME_TITLE);

    let mut correct_count = 0;
    let mut total = 0;
    let mut wrong_hiragana_list = HashSet::new();

    loop {
        let (hiragana, roma) = rand_hiragana();
        let mut ans = String::new();
        
        print!("{}. {} romaji❓︎", total + 1, hiragana);
        io::stdout().flush().expect("<error out>");
        io::stdin().read_line(&mut ans).expect("<error in>");
        ans = ans.trim().to_string();

        if ans == "q" {
            println!("QUIT");
            return;
        }

        if ans == "w" {
            if !wrong_hiragana_list.is_empty() {
                println!("❌ wrong list: {:?}", wrong_hiragana_list);
            } else {
                println!("🈚️");
            }
        } else if ans == roma {
            correct_count += 1;
            total += 1;
            let correct_rate = correct_count * 100 / total;
            println!(" {}➜ {} ✅ 📃 {} / {}, {}%", hiragana, roma, correct_count, total, correct_rate);
        } else {
            // wrong answer
            wrong_hiragana_list.insert(hiragana.to_string());
            total += 1;
            let correct_rate = correct_count * 100 / total;
            println!(" {}➜ {} ❌ 📃 {} / {}, {}%", hiragana, roma, correct_count, total, correct_rate);
        }
    }
}

fn rand_hiragana() -> (&'static str, &'static str) {
    let (hiragana, roma) = HIRAGANA.choose(&mut rand::thread_rng()).unwrap();
    (*hiragana, *roma)
}

const GAME_TITLE: &str = r#"
----------------------
Hiragana Learning Game
    平假名学习游戏
       /\_/\  
      ( o.o ) 
       > ^ <
     あいうえお
----------------------
💡 q: quit, w: wrong list
"#;
