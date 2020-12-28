use crate::common;
use std::collections::HashMap;

use std::fs;

#[derive(Debug)]
enum GrammarRule {
  Term(String),
  NonTerm(Vec<String>),
}

type GrammarRuleSet = HashMap<String, Vec<GrammarRule>>;

fn parse_grammar_rule(text: &str) -> Vec<GrammarRule> {
  let mut res = Vec::new();
  for x in text.split("|").map(|s| s.trim()) {
    if x.starts_with("\"") {
      res.push(GrammarRule::Term(x[1..x.len() - 1].to_string()));
    } else {
      res.push(GrammarRule::NonTerm(
        x.split(" ").map(|s| s.to_string()).collect(),
      ))
    }
  }
  res
}

fn parse_grammar_rules(text: &str) -> (GrammarRuleSet, Vec<String>) {
  let chunks: Vec<Vec<&str>> = text
    .split("\n\n")
    .map(|s| s.trim().split("\n").collect())
    .collect();
  let mut rules = HashMap::new();
  for line in &chunks[0] {
    let parts: Vec<&str> = line.split(": ").collect();
    rules.insert(parts[0].to_string(), parse_grammar_rule(parts[1]));
  }
  (rules, chunks[1].iter().map(|s| s.to_string()).collect())
}

fn is_valid(rules: &GrammarRuleSet, line: &str) -> bool {
  fn match_seq(rules: &GrammarRuleSet, seq: &[String], line: &str, pos: usize) -> Vec<usize> {
    let mut res = Vec::new();
    if seq.is_empty() {
      res.push(pos);
    } else {
      for cpos in match_rule(rules, &seq[0], line, pos) {
        res.extend(match_seq(rules, &seq[1..], line, cpos));
      }
    }
    res
  }

  fn match_rule(rules: &GrammarRuleSet, rule_name: &str, line: &str, pos: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for option in &rules[rule_name] {
      match option {
        GrammarRule::Term(s) if line[pos..].starts_with(s) => res.push(pos + s.len()),
        GrammarRule::NonTerm(opts) => res.extend(match_seq(rules, &opts[0..], line, pos)),
        _ => (),
      };
    }
    res
  }

  let res = match_rule(rules, "0", line, 0);
  if res.len() == 0 {
    false
  } else {
    res[0] == line.len()
  }
}

fn patch_rules(rules: &mut GrammarRuleSet) {
  rules.insert("8".to_string(), parse_grammar_rule("42 | 42 8"));
  rules.insert("11".to_string(), parse_grammar_rule("42 31 | 42 11 31"));
}

pub(crate) fn solution() {
  let (mut rules, strings) =
    parse_grammar_rules(&fs::read_to_string(&common::data_file(19)).unwrap());
  let res1: u32 = strings.iter().map(|s| is_valid(&rules, s) as u32).sum();
  println!("Answer 1: {}", res1);

  patch_rules(&mut rules);
  let res2: u32 = strings.iter().map(|s| is_valid(&rules, s) as u32).sum();
  println!("Answer 2: {}", res2);
}
