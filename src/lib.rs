use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;

pub trait Condorcet<T> {
    fn condorcet_winner(&self) -> Option<&T>;
}

#[derive(Clone, Debug, Eq, PartialOrd, PartialEq)]
struct PreOrder<T>(Vec<T>);

impl<T: Eq> Deref for PreOrder<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Eq> PreOrder<T> {
    pub fn who_is_first<'a>(&self, a: &'a T, b: &'a T) -> Option<&'a T> {
        for it in &self.0 {
            if a == it {
                return Some(a);
            }
            if b == it {
                return Some(b);
            }
        }
        None
    }
}

type Ballot<T> = (usize, PreOrder<T>);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Vote<T: Eq + Hash> {
    candidates: HashSet<T>,
    ballots: Vec<Ballot<T>>,
}

impl<T: Eq + Hash + Clone> Condorcet<T> for Vote<T> {
    fn condorcet_winner(&self) -> Option<&T> {
        let mut res = None;
        'outer: for candidate in &self.candidates {
            let mut s: HashSet<T> = HashSet::new();
            s.insert(candidate.to_owned());
            for other_candidate in self.candidates.difference(&s) {
                let mut scores = (0, 0);
                for (voters, ballot) in &self.ballots {
                    if ballot.who_is_first(candidate, other_candidate).unwrap() == candidate {
                        scores.0 += *voters
                    } else {
                        scores.1 += *voters
                    }
                }
                if scores.0 <= scores.1 {
                    continue 'outer;
                }
            }
            res = Some(candidate);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{Condorcet, PreOrder, Vote};
    use sugars::*;

    #[test]
    fn condorcet_1() {
        assert_eq!(
            Vote {
                candidates: hset!("a", "b", "c",),
                ballots: vec![
                    (35, PreOrder(vec!["a", "b", "c"])),
                    (25, PreOrder(vec!["b", "c", "a"])),
                    (15, PreOrder(vec!["c", "b", "a"])),
                ],
            }
                .condorcet_winner()
                .unwrap(),
            &"b"
        )
    }

    #[test]
    fn condorcet_2() {
        assert_eq!(
            Vote {
                candidates: hset!("a", "b", "c","d"),
                ballots: vec![
                    (42, PreOrder(vec!["a", "b", "c", "d"])),
                    (26, PreOrder(vec!["b", "c", "d", "a"])),
                    (17, PreOrder(vec!["d", "c", "b", "a"])),
                    (15, PreOrder(vec!["c", "d", "b", "a"])),
                ],
            }
                .condorcet_winner()
                .unwrap(),
            &"b"
        )
    }
}


