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

fn main() {
    println!("{}", GAME_TITLE);

    let total: u32 = HIRAGANA.len() as u32;
    let mut i: u32 = 0;
    let mut correct_count: u32 = 0;
    let mut learning_result_map: BTreeMap<&str, i32> = BTreeMap::new();

    loop {
        let (hiragana, roma) = rand_hiragana(&learning_result_map);
        let mut ans = String::new();

        print!("{}. The romaji of {}: ", i, hiragana);
        io::stdout().flush().expect("<error out>");
        io::stdin().read_line(&mut ans).expect("<error in>");
        ans = ans.trim().to_string();

        if ans == "q" {
            println!("QUIT");
            return;
        }

        if ans == "w" {
            print_learning_result(&learning_result_map);
            continue;
        }

        if !learning_result_map.contains_key(hiragana) {
            learning_result_map.insert(hiragana, 0);
        }

        if ans == roma {
            // correct answer
            if let Some(value) = learning_result_map.get_mut(hiragana) {
                if *value == 0 {
                    correct_count += 1;
                }
                *value += 1;
            }
            let correct_rate = correct_count * 100 / total;
            println!(
                " {}➜ {} ✅ 📃 {} / {}, {}%",
                hiragana, roma, correct_count, total, correct_rate
            );
        } else {
            // wrong answer
            if let Some(value) = learning_result_map.get_mut(hiragana) {
                if *value > 0 {
                    correct_count -= 1;
                    *value = 0;
                } else {
                    *value -= 1;
                }
            }
            let correct_rate = correct_count * 100 / total;
            println!(
                " {}➜ {} ❌ 📃 {} / {}, {}%",
                hiragana, roma, correct_count, total, correct_rate
            );
        }
        i += 1;
    }
}

fn rand_hiragana(map: &BTreeMap<&str, i32>) -> (&'static str, &'static str) {
    let mut rng = rand::thread_rng();

    // 70% chance to pick up a hiragana from the wrong list
    if rng.gen_range(0..100) > 70 {
        let mut list: Vec<_> = map.iter().collect();
        list.sort_by_key(|&(_, v)| v);
        for (key, value) in list {
            let mut m = 100;
            if *value < 0 {
                m += (0 - value) * 20; // The words with a higher error rate have a greater probability of being selected for review.
            }
            if rng.gen_range(0..m) > rng.gen_range(0..100) {
                if let Some((hiragana, romaji)) = HIRAGANA.iter().find(|&&(h, _)| h == *key) {
                    return (*hiragana, *romaji);
                } else {
                    println!("Element not found with key: {}", key);
                }
            }
        }
    }

    // to pick up a hiragana from the unchecked list
    let correct_list: Vec<_> = map
        .iter()
        .filter(|&(_, &value)| value > 0)
        .map(|(key, _)| key)
        .collect();

    let unchecked_list: Vec<_> = HIRAGANA
        .iter()
        .filter(|&(h, _)| !correct_list.contains(&h))
        .collect();

    if unchecked_list.len() > 0 {
        let (hiragana, romaji) = unchecked_list.choose(&mut rand::thread_rng()).unwrap();
        return (*hiragana, *romaji);
    }

    *HIRAGANA.choose(&mut rand::thread_rng()).unwrap()
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
