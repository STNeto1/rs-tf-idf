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

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|c| c.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|c| c.is_alphabetic()));
        }

        return Some(self.chop(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

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
            let token = token
                .iter()
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>();
            let count = tf.entry(token).or_insert(0);
            *count += 1;
        }

        file_tf.insert(PathBuf::from(file), tf);
    }

    for (file, tf) in file_tf {
        println!("{} has {} unique tokens", file.display(), tf.len());
    }
}
