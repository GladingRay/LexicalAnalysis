mod regx_process;
mod nfa;
mod dfa;
use regx_process::*;
use nfa::*;
use dfa::*;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;


fn convert_regx_nfa(regx_char_vec: Vec<RegxChar>) -> (Vec<NFAState>, NFA) {
    let mut nfastate_vec: Vec<NFAState> = Vec::new();
    let mut nfa_stack: VecDeque<NFA> = VecDeque::new();
    for regx_char in & regx_char_vec {
        if let RegxChar::NormalChar(n_c) = regx_char {
            nfastate_vec.push(NFAState::new(nfastate_vec.len()));
            nfastate_vec.push(NFAState::new(nfastate_vec.len()));
            let new1 = nfastate_vec.len()-2;
            let new2 = nfastate_vec.len()-1;
            nfastate_vec[new1].add_transform(NFATransform::new(TransformChar::from(*n_c), new2));
            nfa_stack.push_back(NFA::new(new1, new2));
        }
        else if let RegxChar::OperatorChar(o_c) = regx_char {
            match o_c {
                OperatorKind::Cat => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back(){
                                        c
                                    } else {NFA::new(0, 0)};
                    let temp_nfa2 = if let Some(c) = nfa_stack.pop_back(){
                        c
                    } else {NFA::new(0, 0)};
                    nfastate_vec[temp_nfa2.get_end()].add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfa_stack.push_back(NFA::new(temp_nfa2.get_start(), temp_nfa1.get_end()));

                },
                OperatorKind::Closure => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back(){
                        c
                    } else {NFA::new(0, 0)};
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    let new1 = nfastate_vec.len()-2;
                    let new2 = nfastate_vec.len()-1;
                    
                    nfastate_vec[new1].add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfastate_vec[new1].add_transform(NFATransform::new_void(new2));
                    
                    nfastate_vec[temp_nfa1.get_end()].add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfastate_vec[temp_nfa1.get_end()].add_transform(NFATransform::new_void(new2));
                    
                    nfa_stack.push_back(NFA::new(new1, new2));
                },
                OperatorKind::Or => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back(){
                        c
                    } else {NFA::new(0, 0)};
                    let temp_nfa2 = if let Some(c) = nfa_stack.pop_back(){
                        c
                    } else {NFA::new(0, 0)};

                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    let new1 = nfastate_vec.len()-2;
                    let new2 = nfastate_vec.len()-1;

                    nfastate_vec[new1].add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfastate_vec[new1].add_transform(NFATransform::new_void(temp_nfa2.get_start()));

                    nfastate_vec[temp_nfa1.get_end()].add_transform(NFATransform::new_void(new2));
                    nfastate_vec[temp_nfa2.get_end()].add_transform(NFATransform::new_void(new2));

                    nfa_stack.push_back(NFA::new(new1, new2));

                }
            }
        }
    }
    let res_nfa = if let Some(c) = nfa_stack.pop_back() {
                        c
                  } else {
                    NFA::new(0, 0)
                  };
                
    (nfastate_vec, res_nfa)
}

fn get_void_closure (nfastate_vec: &Vec<NFAState>, now: usize, end: usize) -> (HashSet<usize>, bool) {
    let mut closure_set: HashSet<usize> = HashSet::new();
    let mut temp_queue: VecDeque<usize> = VecDeque::new();
    let mut flag: Vec<bool> = Vec::new();
    let mut index = 0;
    let mut is_end = false;
    while index < nfastate_vec.len() {
        flag.push(false);
        index = index + 1;
    }
    temp_queue.push_back(now);
    flag[now] = true;
    while !temp_queue.is_empty() {
        let temp = if let Some(n) = temp_queue.pop_front() {
            n
        }
        else {0};
        closure_set.insert(temp);
        if temp == end {
            is_end = true;
        }
        for transform in nfastate_vec[temp].get_transforms() {
            if let TransformChar::VoidChar = transform.transform {
                if flag[transform.dest] == false {
                    temp_queue.push_back(transform.dest);
                    flag[transform.dest] = true;
                }
            };
        }
    }
    (closure_set, is_end)
}

fn get_set_void_closure(nfastate_vec: &Vec<NFAState>, set: &HashSet<usize>, end: usize) -> (HashSet<usize>, bool) {
    let mut res_closure: HashSet<usize> = HashSet::new();
    let mut res_is_end : bool = false;
    for now in set {
        let (temp_closure, temp_is_end) = get_void_closure(nfastate_vec, *now, end);
        if temp_is_end {
            res_is_end = true;
        }
        res_closure.extend(&temp_closure);
    }
    (res_closure, res_is_end)
}

fn get_move_set(nfastate_vec: &Vec<NFAState>, now_set: &HashSet<usize>) -> HashMap<char, HashSet<usize>> {

    let mut move_map : HashMap<char, HashSet<usize>> = HashMap::new();
    for state in now_set {
        for transform in nfastate_vec[*state].get_transforms() {
            if let TransformChar::NormalChar(c) = transform.transform {
                if !move_map.contains_key(&c) {
                    let mut move_set: HashSet<usize> = HashSet::new();
                    move_set.insert(transform.dest);
                    move_map.insert(c, move_set);
                }
                else {
                    if let Some(temp_set) = move_map.get_mut(&c) {
                        temp_set.insert(transform.dest);
                    };
                }
            };
        }
    }
    move_map
}

fn convert_nfa_dfa (nfastate_vec: &Vec<NFAState>, nfa: &NFA) -> DFA {
    let mut dfa = DFA::new();
    let (closure_set, is_end) = get_void_closure(nfastate_vec, nfa.get_start(), nfa.get_end());
    
    dfa.add_dfa_state(DFAState::new(dfa.gen_next_name(), closure_set, is_end));
    let mut index = 0;
    while index < dfa.gen_next_name() {
        let move_map = get_move_set(nfastate_vec, dfa.get(index).get_nfa_states());
        for (move_char, move_dests) in &move_map {
            let (closure_set, is_end) = get_set_void_closure(nfastate_vec, move_dests, nfa.get_end());
            let dfastate_index = dfa.set_is_exist(&closure_set);
            if dfastate_index == -1 {
                let next_name = dfa.gen_next_name();
                dfa.get(index).add_transform(DFATransform::new(*move_char, next_name));
                dfa.add_dfa_state(DFAState::new(next_name, closure_set, is_end));       
            }
            else {
                dfa.get(index).add_transform(DFATransform::new(*move_char, dfastate_index as usize));
            }
        }
        index = index + 1;
    }
    dfa
}

fn print_nfa(nfa: &NFA, nfastate_vec: &Vec<NFAState>) {
    let mut flag: Vec<bool> = Vec::new();
    let mut index = 0;
    println!("this nfa start on {}, end on {}", nfa.get_start(), nfa.get_end());
    while index < nfastate_vec.len() {
        flag.push(false);
        index = index + 1;
    }
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(nfa.get_start());
    flag[nfa.get_start()] = true;
    while !queue.is_empty() {
        let temp_state = if let Some(c) = queue.pop_front() {
                            c
                        }
                        else {
                            println!("147 error!");
                            nfa.get_start()
                        };
        for transform in nfastate_vec[temp_state].get_transforms() {
            println!("{}-{}->{} ", temp_state, transform.transform.to_char(), transform.dest);
            if flag[transform.dest] == false {
                queue.push_back(transform.dest);
                flag[transform.dest] = true;
            }
        }
    }
}

fn main() {
    let regx = String::from("a(c|b)*");
    let regx_char_vec : Vec<RegxChar> = regx_to_suffix(regx);
    print_regx(&regx_char_vec);
    let (nfastate_vec, nfa) = convert_regx_nfa(regx_char_vec);
    print_nfa(&nfa, &nfastate_vec);
    let dfa = convert_nfa_dfa(&nfastate_vec, &nfa);
    dfa.print_dfa();
}
