#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;

type DocId = i32;
const MIN_DOC_ID: DocId = std::i32::MIN;
const MAX_DOC_ID: DocId = std::i32::MAX;

#[derive(Clone, Debug, PartialEq)]
struct ScoredDoc {
    id: DocId,
    score: f32,
}

struct Term<'a> {
    field: &'a str,
    token: &'a str,
    weight: f32,
}

// used to populate an index in a simple way
type IndexEntry<'a> = (&'a str, &'a str, DocId, f32);

type DensePostingList = Vec<ScoredDoc>;

#[derive(Debug)]
struct InvertedIndexNaive<'a> {
    posting_lists: HashMap<&'a str, HashMap<&'a str, DensePostingList>>,
}

impl<'a> FromIterator<IndexEntry<'a>> for InvertedIndexNaive<'a> {
    fn from_iter<T: IntoIterator<Item = IndexEntry<'a>>>(iter: T) -> Self {
        // insert field -> (token -> (doc id -> score))
        let mut nested_maps = HashMap::new();
        for (field, token, doc_id, payload) in iter {
            let field_entry = nested_maps.entry(field).or_insert(HashMap::new());
            let token_entry = field_entry.entry(token).or_insert(BTreeMap::new());
            token_entry.insert(doc_id, payload);
        }
        let mut posting_lists = HashMap::new();
        // turn BTreeMaps into DesnPostingLists
        for (field, token_map) in nested_maps {
            let field_entry = posting_lists.entry(field).or_insert(HashMap::new());
            for (token, score_map) in token_map {
                let posting_list = score_map
                    .iter()
                    .map(|x| ScoredDoc {
                        id: *x.0,
                        score: *x.1,
                    })
                    .collect();
                field_entry.insert(token, posting_list);
            }
        }
        return InvertedIndexNaive { posting_lists };
    }
}

enum AndState {
    Searching,
    HitEmptyList(usize),
    Done,
}

struct And<'a> {
    state: AndState,
    frontier: Vec<std::slice::Iter<'a, ScoredDoc>>,
    weights: Vec<f32>,
    last_viable_id: i32,
}

impl<'a> And<'a> {
    fn new(index: &'a InvertedIndexNaive, terms: &[Term]) -> Self {
        let mut frontier = Vec::with_capacity(terms.len());
        let mut weights = Vec::with_capacity(terms.len());
        // check to see if we can even find posing lists for all the terms
        for term in terms.iter() {
            let lookup = index
                .posting_lists
                .get(term.field)
                .and_then(|field_result| field_result.get(term.token));

            match lookup {
                None => {
                    return And {
                        state: AndState::Done,
                        frontier: Vec::new(),
                        weights: Vec::new(),
                        last_viable_id: MAX_DOC_ID,
                    }
                }
                Some(list) => {
                    frontier.push(list.iter());
                    weights.push(term.weight);
                }
            }
        }
        And {
            state: AndState::Searching,
            frontier,
            weights,
            last_viable_id: MIN_DOC_ID,
        }
    }
}

impl Iterator for And<'_> {
    type Item = ScoredDoc;

    fn next(&mut self) -> Option<ScoredDoc> {
        let mut matched_doc_count = 0;
        let mut score = 0.0;

        for posting_list_number in (0..self.frontier.len()).cycle() {
            let mut last_viable_id = self.last_viable_id;
            let list_iter = self.frontier.get_mut(posting_list_number).unwrap();
            let mut list_iter = list_iter.skip_while(|doc| doc.id < last_viable_id);
            match list_iter.next() {
                None => match self.state {
                    AndState::Searching => self.state = AndState::HitEmptyList(posting_list_number),
                    AndState::HitEmptyList(i) => {
                        if posting_list_number == i {
                            self.state = AndState::Done;
                        }
                    }
                    AndState::Done => break,
                },
                Some(doc) => {
                    if doc.id == last_viable_id {
                        score += self.weights[posting_list_number] * doc.score;
                        matched_doc_count += 1;
                    } else {
                        matched_doc_count = 0;
                        score = self.weights[posting_list_number] * doc.score;
                        last_viable_id = doc.id;
                    }
                }
            }
            if matched_doc_count == self.frontier.len() - 1 {
                return Some(ScoredDoc {
                    id: last_viable_id,
                    score,
                });
            }
            self.last_viable_id = last_viable_id;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_and() {
        let inverted_index = &InvertedIndexNaive {
            posting_lists: HashMap::new(),
        };
        let term = &vec![Term {
            field: "your",
            token: "mom",
            weight: 9000.1,
        }];
        let and = And::new(inverted_index, term);
        assert_eq!(and.count(), 0);
    }

    #[test]
    fn nonempty_and() {
        let inverted_index: InvertedIndexNaive = [
            ("description", "very", 1, 1.0),
            ("description", "very", 5, 1.0),
            ("description", "human", 2, 1.0),
            ("description", "human", 5, 2.0),
            ("description", "like", 3, 1.0),
            ("description", "like", 5, 3.0),
            ("description", "eyes", 4, 1.0),
            ("description", "eyes", 5, 4.0),
            ("title", "manul", 1, 1.0),
            ("title", "manul", 5, 5.0),
            ("title", "cat", 2, 1.0),
            ("title", "cat", 5, 6.0),
            ("title", "facial", 3, 1.0),
            ("title", "facial", 5, 7.0),
            ("title", "expression", 4, 1.0),
            ("title", "expression", 5, 8.0),
        ]
        .iter()
        .cloned()
        .collect();

        let terms = [
            Term {
                field: "description",
                token: "very",
                weight: 1.0,
            },
            Term {
                field: "description",
                token: "human",
                weight: 10.0,
            },
            Term {
                field: "description",
                token: "like",
                weight: 100.0,
            },
            Term {
                field: "description",
                token: "eyes",
                weight: 1_000.0,
            },
            Term {
                field: "title",
                token: "manul",
                weight: 10_000.0,
            },
            Term {
                field: "title",
                token: "cat",
                weight: 100_000.0,
            },
            Term {
                field: "title",
                token: "facial",
                weight: 1_000_000.0,
            },
            Term {
                field: "title",
                token: "expression",
                weight: 10_000_000.0,
            },
        ];

        let and = And::new(&inverted_index, &terms);
        let actual: Vec<ScoredDoc> = and.collect();
        assert_eq!(
            actual,
            vec![ScoredDoc {
                id: 5,
                score: 87_654_321.0
            }]
        )
    }
}
