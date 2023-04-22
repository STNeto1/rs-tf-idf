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
        .map(|s| s.to_lowercase())
        .map(|s| {
            s.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .filter(|s| s.len() > 1)
        .collect::<Vec<_>>()
}
