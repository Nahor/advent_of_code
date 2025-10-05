use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
pub struct Update(pub Vec<u64>);

impl Update {
    pub fn new(pages: Vec<u64>) -> Self {
        Self(pages)
    }

    pub fn get_mid(&self) -> u64 {
        assert_eq!(self.0.len() % 2, 1);
        self.0[self.0.len() / 2]
    }

    pub fn need_remap(&self, rules: &HashMap<u64, HashSet<u64>>) -> bool {
        let mut seen = HashSet::with_capacity(self.0.len());
        self.0.iter().any(|page| {
            let need_remap = rules
                .get(page)
                .is_some_and(|others| others.iter().any(|other| seen.contains(other)));
            seen.insert(page);
            need_remap
        })
    }

    pub fn is_sorted(&self, rules: &HashMap<u64, HashSet<u64>>) -> bool {
        self.0.is_sorted_by(|page, other| {
            // `page` is before `other`:
            // - if there is a rule that says so
            // - or if there are no rule that say that `other` is before `page`
            //
            // This works if the rule list is exhaustive or specially constructed,
            // since `is_sorted_by()` will only compare consecutive pairs of pages.

            rules.get(page).is_some_and(|others| others.contains(other))
                || rules.get(other).is_none_or(|pages| !pages.contains(page))
        })
    }

    pub fn sort(mut self, rules: &HashMap<u64, HashSet<u64>>) -> Self {
        self.0.sort_by(|page, other| {
            // This works if the rule list is exhaustive, i.e. it contains every
            // possible pair of pages for an incorrectly-ordered update.
            if rules.get(page).is_some_and(|others| others.contains(other)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        self
    }
}

#[derive(Debug)]
pub struct Document {
    pub rules: HashMap<u64, HashSet<u64>>,
    pub updates: Vec<Update>,
}

impl Document {}
