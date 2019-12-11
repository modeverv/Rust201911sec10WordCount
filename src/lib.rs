//! libです
#![warn(missing_docs)]
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
/**
count関数です
 */
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut fregs = HashMap::new();
    for line in input.lines() {
        use crate::CountOption::*;
        let line = line.unwrap();
        match option {
            Char => {
                for c in line.chars() {
                    *fregs.entry(c.to_string()).or_insert(0) += 1;
                }
            },
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *fregs.entry(word).or_insert(0) += 1;
                }
            },
            Line => {
                *fregs.entry(line.to_string()).or_insert(0) += 1;
            }
        }
    }
    fregs
}
/// 列挙子です。
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum CountOption {
    /// 文字
    Char,
    /// 単語
    Word,
    /// 行
    Line,
}
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

#[test]
fn word_count_works() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(),1);
    exp.insert("bb".to_string(),2);
    assert_eq!(count(Cursor::new("aa bb bb"),CountOption::Word),exp);
}

