use std::{collections::HashMap, path::PathBuf};

type TF = HashMap<String, usize>;
type FileTF = HashMap<PathBuf, TF>;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while self.content.len() > n && predicate(&self.content[n]) {
            n += 1;
        }

        self.chop(n)
    }

    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|c| c.is_numeric()).iter().collect());
        }

        if self.content[0].is_alphabetic() {
            return Some(
                self.chop_while(|c| c.is_alphabetic())
                    .iter()
                    .map(|c| c.to_ascii_uppercase())
                    .collect(),
            );
        }

        return Some(self.chop(1).iter().collect());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() {
    // let all_documents: HashMap<Path, HashMap<String, i32>> = HashMap::new();

    let mut file_tf: FileTF = HashMap::new();

    let files = vec!["unix", "posix", "c"];

    for file in files {
        let content = std::fs::read_to_string(format!("assets/{}.txt", file))
            .unwrap()
            .chars()
            .collect::<Vec<_>>();

        let mut tf = TF::new();

        for token in Lexer::new(&content) {
            let count = tf.entry(token).or_insert(0);
            *count += 1;
        }

        file_tf.insert(PathBuf::from(file), tf);
    }

    let term_to_search = "unix computing language POSIX";
    let mut result = Vec::<(&PathBuf, f32)>::new();

    for (file, tf) in &file_tf {
        let mut total_sum = 0f32;
        for token in Lexer::new(term_to_search.chars().collect::<Vec<_>>().as_slice()) {
            let token_tf_rating = tf_rating(&token, tf);
            let token_idf_rating = idf_rating(&token, &file_tf);

            total_sum += token_tf_rating * token_idf_rating;
        }

        result.push((file, total_sum));
    }

    result.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for (file, score) in result {
        println!("{}: {}", file.display(), score);
    }
}

fn tf_rating(term: &str, document: &TF) -> f32 {
    let top = document.get(term).clone().unwrap_or(&0);
    let bottom = document.values().sum::<usize>();

    return *top as f32 / bottom as f32;
}

fn idf_rating(term: &str, all_documents: &FileTF) -> f32 {
    let n = all_documents.len();
    let df = all_documents
        .values()
        .filter(|document| document.contains_key(term))
        .count()
        .max(1);

    return (n as f32 / df as f32).log10();
}
