#![allow(dead_code)]

use ammonia;
use std::collections::HashSet;

pub fn sanitize_input(input: &str) -> String {
    input
        .trim()
        .chars()
        .filter(|c| !c.is_control() || matches!(c, '\t' | '\n' | '\r'))
        .collect()
}

pub fn validate_length(input: &str, min: usize, max: usize) -> bool {
    let len = input.chars().count();
    len >= min && len <= max
}

pub fn contains_dangerous_patterns(input: &str) -> bool {
    let lower = input.to_lowercase();
    [
        "<script",
        "javascript:",
        "onerror=",
        "onclick=",
        "union select",
        "drop table",
    ]
    .iter()
    .any(|p| lower.contains(p))
}

#[allow(dead_code)]
pub fn sanitize_html(input: &str) -> String {
    let tags: HashSet<&str> = ["b", "i", "strong", "em", "p", "br", "ul", "ol", "li"]
        .into_iter()
        .collect();

    ammonia::Builder::new().tags(tags).clean(input).to_string()
}

pub fn strip_html(input: &str) -> String {
    ammonia::Builder::new()
        .tags(HashSet::new())
        .clean(input)
        .to_string()
}
