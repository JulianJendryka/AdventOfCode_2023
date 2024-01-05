use std::collections::{HashMap, VecDeque};

use super::lense;

#[derive(Clone, Debug)]
pub struct Box {
    pub lenses: VecDeque<lense::Lense>,
    lenses_present: HashMap<String, usize>, // maps lense_label to pos_in_box
}

impl Box {
    pub fn create_empty() -> Box {
        Box {
            lenses: VecDeque::new(),
            lenses_present: HashMap::new(),
        }
    }

    pub fn handle_lense(&mut self, lense: lense::Lense) {
        match lense.do_insert {
            true => self.insert(lense),
            false => self.remove(lense),
        }
    }

    fn insert(&mut self, lense: lense::Lense) {
        match self.lenses_present.contains_key(&lense.label) {
            true => {
                // alrdy existing lense, update focal length
                let lense_index = self.lenses_present[&lense.label];
                self.lenses[lense_index].focal_length = lense.focal_length;
            }
            false => {
                self.lenses_present
                    .insert(lense.label.clone(), self.lenses.len());
                self.lenses.push_back(lense)
            }
        }
    }

    fn remove(&mut self, lense: lense::Lense) {
        match self.lenses_present.contains_key(&lense.label) {
            false => (), // not existing lense, do nothing
            true => {
                let lense_index = self.lenses_present[&lense.label];
                self.lenses.remove(lense_index);
                self.lenses_present.remove(&lense.label);

                // decrement the indices of all following lenses
                if !self.lenses_present.is_empty() {
                    for (_, index) in self.lenses_present.iter_mut() {
                        if *index > lense_index {
                            *index -= 1;
                        }
                    }
                }
            }
        }
    }
}
