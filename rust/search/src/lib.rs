#![allow(dead_code)]
use std::collections::HashMap;

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

type DensePostingList = Vec<ScoredDoc>;

#[derive(Debug)]
struct InvertedIndexNaive<'a> {
    posting_lists: HashMap<&'a str, HashMap<&'a str, DensePostingList>>,
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
        let description_list: HashMap<&str, DensePostingList> = [
            (
                "very",
                vec![
                    ScoredDoc { id: 1, score: 1.0 },
                    ScoredDoc { id: 5, score: 1.0 },
                ],
            ),
            (
                "human",
                vec![
                    ScoredDoc { id: 2, score: 1.0 },
                    ScoredDoc { id: 5, score: 2.0 },
                ],
            ),
            (
                "like",
                vec![
                    ScoredDoc { id: 3, score: 1.0 },
                    ScoredDoc { id: 5, score: 3.0 },
                ],
            ),
            (
                "eyes",
                vec![
                    ScoredDoc { id: 4, score: 1.0 },
                    ScoredDoc { id: 5, score: 4.0 },
                ],
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let title_list: HashMap<&str, DensePostingList> = [
            (
                "manul",
                vec![
                    ScoredDoc { id: 1, score: 1.0 },
                    ScoredDoc { id: 5, score: 5.0 },
                ],
            ),
            (
                "cat",
                vec![
                    ScoredDoc { id: 2, score: 1.0 },
                    ScoredDoc { id: 5, score: 6.0 },
                ],
            ),
            (
                "facial",
                vec![
                    ScoredDoc { id: 3, score: 1.0 },
                    ScoredDoc { id: 5, score: 7.0 },
                ],
            ),
            (
                "expression",
                vec![
                    ScoredDoc { id: 4, score: 1.0 },
                    ScoredDoc { id: 5, score: 8.0 },
                ],
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let posting_lists = [("title", title_list), ("description", description_list)]
            .iter()
            .cloned()
            .collect();
        let inverted_index = InvertedIndexNaive {
            posting_lists,
        };

        let terms = vec![
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
