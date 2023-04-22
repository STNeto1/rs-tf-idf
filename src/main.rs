use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("assets/unix.txt").unwrap();
    let tokens = tokenizer(&content);

    let mut map: HashMap<&str, i32> = std::collections::HashMap::new();
    tokens.iter().for_each(|token| {
        let count = map.entry(token).or_insert(0);
        *count += 1;
    });

    println!("{:?}", map)
}

fn tokenizer(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .map(|s| s.to_lowercase()) // convert to lowercase
        .filter(|s| {
            s.chars()
                .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        }) // filter out non-alphabetic characters
        .filter(|s| s.len() > 1) // filter out single characters
        .collect::<Vec<_>>()
}
