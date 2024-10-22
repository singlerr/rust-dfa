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

pub struct State<S, I> {
    state_transitions: HashMap<I, S>,
}

pub struct DFA<S, I> {
    languages: Box<[I]>,
    states: HashMap<S, State<S, I>>,
    final_states: Box<[S]>,
    state: S,
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

    pub fn feed<Q: ?Sized>(&mut self, i: &Q) -> Result<&S>
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
        state: 0,
    };
}
