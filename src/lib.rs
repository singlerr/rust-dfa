use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

mod dfa;

#[macro_export]
macro_rules! states {
    (let ) => {};
}

pub enum Result<S> {
    Ok(S),
    End(S),
    Err,
}

trait Comp: Eq + Hash {}

/// Check provided character is in a language set
trait LanguageChecker {
    fn test(&self, input: &'static str) -> bool;
}

pub struct State<S, I> {
    state_transitions: HashMap<I, S>,
}

pub struct DFA<S, I> {
    languages: Box<[I]>,
    states: HashMap<S, State<S, I>>,
    input_checker: Box<dyn LanguageChecker>,
    final_states: Box<[S]>,
    state: S,
}

struct RegexChecker {
    regex: Regex,
}

impl LanguageChecker for RegexChecker {
    fn test(&self, input: &'static str) -> bool {
        self.regex.is_match(input)
    }
}

impl RegexChecker {
    pub fn from_regex(regex: Regex) -> Self {
        RegexChecker { regex }
    }

    pub fn from_pattern(pattern: &str) -> core::result::Result<RegexChecker, regex::Error> {
        let compiled_regex = Regex::new(pattern)?;
        Ok(RegexChecker {
            regex: compiled_regex,
        })
    }
}

impl<S, I> DFA<S, I>
where
    S: Hash + Eq,
    I: Hash + Eq,
{
    fn is_final(&self, s: &S) -> bool {
        for final_state in (*self.final_states).iter() {
            if final_state == s {
                return true;
            }
        }
        false
    }

    pub fn consume<Q: ?Sized>(&mut self, i: &Q) -> Result<&S>
    where
        I: Borrow<Q>,
        Q: Hash + Eq,
    {
        let s = self.states.get(&self.state);
        match s {
            Some(s) => {
                if let Some(s) = s.state_transitions.get(i) {
                    if self.is_final(s) {
                        Result::End(s)
                    } else {
                        Result::Ok(s)
                    }
                } else {
                    Result::Err
                }
            }
            None => Result::Err,
        }
    }
}

fn a() {
    let dfa: DFA<u8, char> = DFA {
        languages: Box::new(['a'; 10]),
        states: HashMap::new(),
        final_states: Box::new([10]),
        input_checker: Box::new(RegexChecker::from_pattern("").unwrap()),
        state: 0,
    };
}
