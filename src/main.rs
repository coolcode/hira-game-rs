use rand::seq::SliceRandom;
use std::collections::BTreeMap;
// use std::collections::HashSet;
use rand::Rng;
use std::io;
use std::io::Write;

const HIRAGANA: [(&str, &str); 46] = [
    ("あ", "a"),
    ("い", "i"),
    ("う", "u"),
    ("え", "e"),
    ("お", "o"),
    ("か", "ka"),
    ("き", "ki"),
    ("く", "ku"),
    ("け", "ke"),
    ("こ", "ko"),
    ("さ", "sa"),
    ("し", "shi"),
    ("す", "su"),
    ("せ", "se"),
    ("そ", "so"),
    ("た", "ta"),
    ("ち", "chi"),
    ("つ", "tsu"),
    ("て", "te"),
    ("と", "to"),
    ("な", "na"),
    ("に", "ni"),
    ("ぬ", "nu"),
    ("ね", "ne"),
    ("の", "no"),
    ("は", "ha"),
    ("ひ", "hi"),
    ("ふ", "fu"),
    ("へ", "he"),
    ("ほ", "ho"),
    ("ま", "ma"),
    ("み", "mi"),
    ("む", "mu"),
    ("め", "me"),
    ("も", "mo"),
    ("や", "ya"),
    ("ゆ", "yu"),
    ("よ", "yo"),
    ("ら", "ra"),
    ("り", "ri"),
    ("る", "ru"),
    ("れ", "re"),
    ("ろ", "ro"),
    ("わ", "wa"),
    ("を", "wo"),
    ("ん", "n"),
];

const KATAKANA: [(&str, &str); 46] = [
    ("ア", "a"),
    ("イ", "i"),
    ("ウ", "u"),
    ("エ", "e"),
    ("オ", "o"),
    ("カ", "ka"),
    ("キ", "ki"),
    ("ク", "ku"),
    ("ケ", "ke"),
    ("コ", "ko"),
    ("サ", "sa"),
    ("シ", "shi"),
    ("ス", "su"),
    ("セ", "se"),
    ("ソ", "so"),
    ("タ", "ta"),
    ("チ", "chi"),
    ("ツ", "tsu"),
    ("テ", "te"),
    ("ト", "to"),
    ("ナ", "na"),
    ("ニ", "ni"),
    ("ヌ", "nu"),
    ("ネ", "ne"),
    ("ノ", "no"),
    ("ハ", "ha"),
    ("ヒ", "hi"),
    ("フ", "fu"),
    ("ヘ", "he"),
    ("ホ", "ho"),
    ("マ", "ma"),
    ("ミ", "mi"),
    ("ム", "mu"),
    ("メ", "me"),
    ("モ", "mo"),
    ("ヤ", "ya"),
    ("ユ", "yu"),
    ("ヨ", "yo"),
    ("ラ", "ra"),
    ("リ", "ri"),
    ("ル", "ru"),
    ("レ", "re"),
    ("ロ", "ro"),
    ("ワ", "wa"),
    ("ヲ", "wo"),
    ("ン", "n"),
];

fn main() {
    println!("{}", GAME_TITLE);

    let mut total = 0;
    let mut i = 0;
    let mut correct_count = 0;
    let mut learning_result_map: BTreeMap<&str, i32> = BTreeMap::new();
    let mut learning_type = 1; // 1: HIRAGANA, 2: KATAKANA

    loop {
        let selected_symbols = match learning_type {
            1 => &HIRAGANA,
            2 => &KATAKANA,
            _ => &HIRAGANA, // Default to HIRAGANA if learning_type is not 1 or 2
        };

        let (symbol, roma) = rand_symbol(selected_symbols, &learning_result_map);
        let mut ans = String::new();

        print!("{}.[romaji] {}: ", i + 1, symbol);
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read input");
        ans = ans.trim().to_string();

        if ans == "q" {
            println!("QUIT");
            return;
        } else if ans == "w" {
            print_learning_result(&learning_result_map);
            continue;
        } else if ans == "1" {
            learning_type = 1;
            i = 0;
            correct_count = 0;
            learning_result_map.clear();
            continue;
        } else if ans == "2" {
            learning_type = 2;
            i = 0;
            correct_count = 0;
            learning_result_map.clear();
            continue;
        }

        if !learning_result_map.contains_key(symbol) {
            learning_result_map.insert(symbol, 0);
        }

        if ans == roma {
            // correct answer
            if let Some(value) = learning_result_map.get_mut(symbol) {
                if *value == 0 {
                    correct_count += 1;
                }
                *value += 1;
            }
            let correct_rate = generate_rate_bar(correct_count * 100 / total);
            let nice_space = generate_space(i + 1);
            println!(
                "{}{}➜ {} ✅ 📃 {} / {}, {}",
                nice_space, symbol, roma, correct_count, total, correct_rate
            );
        } else {
            // wrong answer
            if let Some(value) = learning_result_map.get_mut(symbol) {
                if *value > 0 {
                    correct_count -= 1;
                    *value = 0;
                } else {
                    *value -= 1;
                }
            }
            let correct_rate = generate_rate_bar(correct_count * 100 / total);
            let nice_space = generate_space(i + 1);
            println!(
                "{}{}➜ {} ❌ 📃 {} / {}, {}",
                nice_space, symbol, roma, correct_count, total, correct_rate
            );
        }
        i += 1;
        total = selected_symbols.len() as u32;
    }
}

fn rand_symbol<'a>(
    symbols: &'a [(&'a str, &'a str); 46],
    map: &BTreeMap<&str, i32>,
) -> (&'a str, &'a str) {
    let mut rng = rand::thread_rng();

    // 70% chance to pick up a symbol from the wrong list
    if rng.gen_range(0..100) > 70 {
        let mut list: Vec<_> = map.iter().collect();
        list.sort_by_key(|&(_, v)| v);
        for (key, value) in list {
            let mut m = 100;
            if *value < 0 {
                m += (0 - value) * 20; // The words with a higher error rate have a greater probability of being selected for review.
            }
            if rng.gen_range(0..m) > rng.gen_range(0..100) {
                if let Some((symbol, romaji)) = symbols.iter().find(|&&(h, _)| h == *key) {
                    return (*symbol, *romaji);
                } else {
                    println!("Element not found with key: {}", key);
                }
            }
        }
    }

    // to pick up a symbol from the unchecked list
    let correct_list: Vec<_> = map
        .iter()
        .filter(|&(_, &value)| value > 0)
        .map(|(key, _)| key)
        .collect();

    let unchecked_list: Vec<_> = symbols
        .iter()
        .filter(|&(h, _)| !correct_list.contains(&h))
        .collect();

    if unchecked_list.len() > 0 {
        let (symbol, romaji) = unchecked_list.choose(&mut rand::thread_rng()).unwrap();
        return (*symbol, *romaji);
    }

    let (s, r) = symbols.choose(&mut rand::thread_rng()).unwrap();
    (*s, *r)
}

fn print_learning_result(map: &BTreeMap<&str, i32>) {
    // Iterate over the map and filter elements with values less than 0
    let mut wrong_list: Vec<_> = map.iter().filter(|&(_, &value)| value < 0).collect();

    // Sort by value
    wrong_list.sort_by_key(|&(_, v)| v);

    // Check if there are elements in the wrong list
    if wrong_list.is_empty() {
        println!("🈚️");
    } else {
        println!("❌ wrong list:");
        for (key, value) in wrong_list {
            println!("{}: {}", key, 0 - value);
        }
    }
}

fn generate_rate_bar(percentage: u32) -> String {
    let num_blocks = percentage / 10;
    let mut bar = String::new();
    for _ in 0..num_blocks {
        bar.push('█');
    }
    format!("{}% {}", percentage, bar)
}

fn generate_space(i: u32) -> String {
    let num_spaces = 11 + (i as f64).log10() as u32;
    let mut space = String::new();
    for _ in 0..num_spaces {
        space.push(' ');
    }
    space
}

const GAME_TITLE: &str = r#"
-------------------------------
HIRAGANA/KATAKANA Learning Game
    平假名/片假名学习游戏
           /\_/\  
          ( o.o ) 
           > ^ <
          あいうえお
-------------------------------
💡 q: quit
   w: wrong list
   1: HIRAGANA
   2: KATAKANA
"#;
