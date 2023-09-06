use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::BTreeMap;
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
    // Display the game title and instructions
    println!("{}", GAME_TITLE);

    let mut i = 0;
    let mut correct_count = 0;
    let mut learning_result_map: BTreeMap<&str, i32> = BTreeMap::new();
    let mut learning_type = 1; // 1: HIRAGANA, 2: KATAKANA

    loop {
        // Select the symbols based on the current learning type
        let selected_symbols = match learning_type {
            1 => &HIRAGANA,
            2 => &KATAKANA,
            _ => &HIRAGANA, // Default to HIRAGANA if learning_type is not 1 or 2
        };

        // Get a random symbol and its romaji
        let (symbol, roma) = rand_symbol(selected_symbols, &learning_result_map);

        // Read the user's answer
        let ans = read_user_input(i, &symbol);

        // Handle user commands
        match &ans[..] {
            "q" => {
                println!("QUIT");
                return;
            }
            "w" => {
                print_learning_result(&learning_result_map);
                continue;
            }
            "1" => {
                learning_type = 1;
                i = 0;
                correct_count = 0;
                learning_result_map.clear();
                continue;
            }
            "2" => {
                learning_type = 2;
                i = 0;
                correct_count = 0;
                learning_result_map.clear();
                continue;
            }
            _ => {
                if !learning_result_map.contains_key(symbol) {
                    learning_result_map.insert(symbol, 0);
                }
                handle_answer(
                    i,
                    &ans,
                    &symbol,
                    &roma,
                    &mut correct_count,
                    &mut learning_result_map,
                    selected_symbols.len() as u32,
                );
            }
        }

        i += 1;
    }
}

fn rand_symbol<'a>(
    symbols: &'a [(&'a str, &'a str); 46],
    map: &BTreeMap<&str, i32>,
) -> (&'a str, &'a str) {
    let mut rng = rand::thread_rng();

    // Calculate the maximum value for random selection
    let max_selection_value = if !map.is_empty() {
        // If there are symbols with errors, increase the selection range
        120
    } else {
        100
    };

    // Generate a random number to determine if a symbol with errors should be selected
    let select_with_errors = rng.gen_range(0..100) > 70;

    // Check symbols with errors if they should be selected
    if select_with_errors {
        // Create a list of symbols sorted by error count
        let mut sorted_symbols: Vec<_> = map.iter().collect();
        sorted_symbols.sort_by_key(|&(_, v)| v);

        for (key, value) in sorted_symbols {
            let mut selection_range = max_selection_value;

            // Increase the selection range for symbols with errors
            if *value < 0 {
                selection_range += (-*value) * 20;
            }

            // Check if the symbol should be selected
            if rng.gen_range(0..selection_range) > rng.gen_range(0..100) {
                if let Some((symbol, romaji)) = symbols.iter().find(|&&(h, _)| h == *key) {
                    return (*symbol, *romaji);
                } else {
                    println!("Element not found with key: {}", key);
                }
            }
        }
    }

    // Select a symbol from the unchecked list
    let correct_list: Vec<_> = map
        .iter()
        .filter(|&(_, &value)| value > 0)
        .map(|(key, _)| key)
        .collect();
    let unchecked_list: Vec<_> = symbols
        .iter()
        .filter(|&(h, _)| !correct_list.contains(&h))
        .collect();

    if let Some((symbol, romaji)) = unchecked_list.choose(&mut rng) {
        return (*symbol, *romaji);
    }

    // If no unchecked symbols are left, select a random symbol
    if let Some((symbol, romaji)) = symbols.choose(&mut rng) {
        return (*symbol, *romaji);
    }

    // Default fallback (shouldn't reach this point)
    ("", "")
}

fn read_user_input(i: u32, symbol: &str) -> String {
    print!("{}.[romaji] {}: ", i + 1, symbol);
    io::stdout().flush().expect("<error out>");

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("<error in>");

    ans.trim().to_string()
}

fn handle_answer(
    i: u32,
    ans: &str,
    symbol: &str,
    roma: &str,
    correct_count: &mut u32,
    learning_result_map: &mut BTreeMap<&str, i32>,
    total: u32,
) {
    if ans == roma {
        // Correct answer
        if let Some(value) = learning_result_map.get_mut(symbol) {
            if *value == 0 {
                *correct_count += 1;
            }
            *value += 1;
        }
        let correct_rate = generate_rate_bar(*correct_count * 100 / total);
        let nice_space = generate_space(i + 1);
        println!(
            "{}{}➜ {} ✅ 📃 {} / {}, {}",
            nice_space, symbol, roma, *correct_count, total, correct_rate
        );
    } else {
        // Wrong answer
        if let Some(value) = learning_result_map.get_mut(symbol) {
            if *value > 0 {
                *correct_count -= 1;
                *value = 0;
            } else {
                *value -= 1;
            }
        }
        let correct_rate = generate_rate_bar(*correct_count * 100 / total);
        let nice_space = generate_space(i + 1);
        println!(
            "{}{}➜ {} ❌ 📃 {} / {}, {}",
            nice_space, symbol, roma, *correct_count, total, correct_rate
        );
    }
}

fn print_learning_result(map: &BTreeMap<&str, i32>) {
    // Filter and sort elements with negative values (symbols with errors)
    let mut wrong_list: Vec<_> = map.iter().filter(|&(_, &value)| value < 0).collect();
    wrong_list.sort_by_key(|&(_, v)| v);

    if wrong_list.is_empty() {
        println!("🈚️");
    } else {
        println!("❌ wrong list:");
        for (key, value) in wrong_list {
            println!("{}: {}", key, 0 - *value);
        }
    }
}

fn generate_rate_bar(percentage: u32) -> String {
    let num_blocks = percentage / 10;
    let bar: String = (0..num_blocks).map(|_| "█").collect();
    format!("{}% {}", percentage, bar)
}

fn generate_space(i: u32) -> String {
    let num_spaces = 11 + (i as f64).log10() as u32;
    " ".repeat(num_spaces as usize)
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
