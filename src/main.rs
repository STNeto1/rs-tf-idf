use std::collections::HashMap;

#[derive(Debug, Default)]
struct Metadata {
    tf: i32,
    idf: f32,
    tf_idf: f32,
}

fn main() {
    let files = vec!["unix", "posix", "c"];

    let content = std::fs::read_to_string("assets/unix.txt").unwrap();
    let tokens = tokenizer(&content);

    let token_map: HashMap<&str, Vec<String>> = files
        .clone()
        .into_iter()
        .map(|file| {
            let content = std::fs::read_to_string(format!("assets/{}.txt", file)).unwrap();
            let tokens = tokenizer(&content);
            (file, tokens)
        })
        .collect();

    let mut map: HashMap<&str, Metadata> = std::collections::HashMap::new();

    // calculate term frequency
    tokens.iter().for_each(|token| {
        let metadata = map.entry(token).or_insert(Metadata::default());

        *metadata = Metadata {
            tf: metadata.tf + 1,
            idf: 0.0,
            tf_idf: 0.0,
        };
    });

    // calculate inverse document frequency
    tokens.iter().for_each(|token| {
        let metadata = map.entry(token).or_insert(Metadata::default());

        let files_with_token = token_map
            .iter()
            .filter(|(_, tokens)| tokens.contains(&token.to_string()))
            .count();

        let idf = (files.len() as f32 / files_with_token as f32).ln();

        *metadata = Metadata {
            tf: metadata.tf,
            idf,
            tf_idf: metadata.tf as f32 * idf,
        };
    });

    // sort map by tf_idf and print
    let mut vec: Vec<(_, _)> = map.iter().collect();
    vec.sort_by(|(_, a), (_, b)| b.tf_idf.partial_cmp(&a.tf_idf).unwrap());

    vec.iter().for_each(|(token, metadata)| {
        println!("{}: {:?}", token, metadata);
    });
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
