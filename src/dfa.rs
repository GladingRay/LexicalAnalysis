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
    pub fn get_nfa_states(&self) -> &HashSet<usize> {
        &self.nfa_states
    }
}

pub struct DFA {
    dfastate_vec : Vec<DFAState>,
}

impl DFA {
    pub fn new () -> DFA {
        DFA {
            dfastate_vec: Vec::new()
        }
    }
    pub fn add_dfa_state(&mut self, dfa_state: DFAState) {
        self.dfastate_vec.push(dfa_state);
    }
    pub fn gen_next_name(&self) -> usize {
        self.dfastate_vec.len()
    }
    pub fn get(&mut self, index: usize) -> &mut DFAState {
        &mut self.dfastate_vec[index]
    }
    pub fn set_is_exist(&self, set: &HashSet<usize>) -> isize {
        for state in &self.dfastate_vec {
            if state.set_is_eq(set) {
                return state.name as isize;
            }
        };
        -1
    }
    pub fn print_dfa(&self) {
        println!("DFA 部分:");
        println!("dfa state number = {}", self.dfastate_vec.len());
        for dfa_state in &self.dfastate_vec {
            print!("{}{:?}", dfa_state.name, dfa_state.nfa_states);
            if dfa_state.is_end {
                print!(" end ");
            }
            else {
                print!("  ")
            }
            for trans in &dfa_state.transforms {
                print!("|{}->{}|",trans.transform_char, trans.dest);
            }
            println!();
        }
    }

    pub fn check_string(&self, s: &str) -> bool {
        let mut state = 0;
        println!("check 部分:");
        for c in s.chars() {
            print!("{}-{}->", state, c);
            if self.dfastate_vec[state].transforms.len() == 0 {
                return false;
            }
            let mut flag = true;
            for trans in &self.dfastate_vec[state].transforms {
                if trans.transform_char == c {
                    state = trans.dest;
                    flag = false;
                    break;
                }
            }
            if flag { 
                println!();
                return false;
            }
        }
        println!("{}",state);
        self.dfastate_vec[state].is_end
    }
}

