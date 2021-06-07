use std::collections::HashSet;

/*
    构造DFA部分
*/

pub struct DFATransform {
    pub transform_char: char,
    pub dest: usize
}

impl DFATransform {
    pub fn new (transform_char : char, dest: usize) -> DFATransform {
        DFATransform {
            transform_char,
            dest
        }
    }
}

pub struct DFAState {
    name : usize,
    is_end : bool,
    nfa_states : HashSet<usize>,
    transforms : Vec<DFATransform>
}

impl DFAState {
    pub fn new (name : usize, nfa_states : HashSet<usize>, is_end : bool) -> DFAState {
        DFAState {
            name,
            is_end,
            nfa_states,
            transforms : Vec::new()
        }
    }
    pub fn add_transform(&mut self, dfa_trans: DFATransform) {
        self.transforms.push(dfa_trans);
    }
    pub fn set_is_eq(&self, set: &HashSet<usize>) -> bool {
        self.nfa_states.eq(set)
    }
}

pub struct DFA {
    dfastate_vec : Vec<DFAState>,
    start : usize
}

impl DFA {
    pub fn new () -> DFA {
        DFA {
            dfastate_vec: Vec::new(),
            start: 0
        }
    }
    pub fn add_dfa_state(&mut self, dfa_state: DFAState) {
        self.dfastate_vec.push(dfa_state);
    }
    pub fn gen_next_name(&self) -> usize {
        self.dfastate_vec.len()
    }
    pub fn get(&self, index: usize) -> &DFAState {
        &self.dfastate_vec[index]
    }
    pub fn set_is_exist(&self, set: &HashSet<usize>) -> isize {
        for state in &self.dfastate_vec {
            if state.set_is_eq(set) {
                return state.name as isize;
            }
        };
        -1
    }
}

