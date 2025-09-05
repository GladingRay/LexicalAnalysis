mod dfa;
mod nfa;
mod regx_process;
use dfa::*;
use nfa::*;
use regx_process::*;
use std::collections::VecDeque;
use std::io::stdin;

fn convert_regx_nfa(regx_char_vec: Vec<RegxChar>) -> (Vec<NFAState>, NFA) {
    let mut nfastate_vec: Vec<NFAState> = Vec::new();
    let mut nfa_stack: VecDeque<NFA> = VecDeque::new();
    for regx_char in &regx_char_vec {
        if let RegxChar::NormalChar(n_c) = regx_char {
            nfastate_vec.push(NFAState::new(nfastate_vec.len()));
            nfastate_vec.push(NFAState::new(nfastate_vec.len()));
            let new1 = nfastate_vec.len() - 2;
            let new2 = nfastate_vec.len() - 1;
            nfastate_vec[new1].add_transform(NFATransform::new(TransformChar::from(*n_c), new2));
            nfa_stack.push_back(NFA::new(new1, new2));
        } else if let RegxChar::OperatorChar(o_c) = regx_char {
            match o_c {
                OperatorKind::Cat => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back() {
                        c
                    } else {
                        NFA::new(0, 0)
                    };
                    let temp_nfa2 = if let Some(c) = nfa_stack.pop_back() {
                        c
                    } else {
                        NFA::new(0, 0)
                    };
                    nfastate_vec[temp_nfa2.get_end()]
                        .add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfa_stack.push_back(NFA::new(temp_nfa2.get_start(), temp_nfa1.get_end()));
                }
                OperatorKind::Closure => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back() {
                        c
                    } else {
                        NFA::new(0, 0)
                    };
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    let new1 = nfastate_vec.len() - 2;
                    let new2 = nfastate_vec.len() - 1;

                    nfastate_vec[new1].add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfastate_vec[new1].add_transform(NFATransform::new_void(new2));

                    nfastate_vec[temp_nfa1.get_end()]
                        .add_transform(NFATransform::new_void(temp_nfa1.get_start()));
                    nfastate_vec[temp_nfa1.get_end()].add_transform(NFATransform::new_void(new2));

                    nfa_stack.push_back(NFA::new(new1, new2));
                }
                OperatorKind::Or => {
                    let temp_nfa1 = if let Some(c) = nfa_stack.pop_back() {
                        c
                    } else {
                        NFA::new(0, 0)
                    };
                    let temp_nfa2 = if let Some(c) = nfa_stack.pop_back() {
                        c
                    } else {
                        NFA::new(0, 0)
                    };

                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    nfastate_vec.push(NFAState::new(nfastate_vec.len()));
                    let new1 = nfastate_vec.len() - 2;
                    let new2 = nfastate_vec.len() - 1;

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

fn convert_nfa_dfa(nfastate_vec: &Vec<NFAState>, nfa: &NFA) -> DFA {
    let mut dfa = DFA::new();
    let (closure_set, is_end) = get_void_closure(nfastate_vec, nfa.get_start(), nfa.get_end());

    dfa.add_dfa_state(DFAState::new(dfa.gen_next_name(), closure_set, is_end));
    let mut index = 0;
    while index < dfa.gen_next_name() {
        let move_map = get_move_set(nfastate_vec, dfa.get(index).get_nfa_states());
        for (move_char, move_dests) in &move_map {
            let (closure_set, is_end) =
                get_set_void_closure(nfastate_vec, move_dests, nfa.get_end());
            let dfastate_index = dfa.set_is_exist(&closure_set);
            if dfastate_index == -1 {
                let next_name = dfa.gen_next_name();
                dfa.get(index)
                    .add_transform(DFATransform::new(*move_char, next_name));
                dfa.add_dfa_state(DFAState::new(next_name, closure_set, is_end));
            } else {
                dfa.get(index)
                    .add_transform(DFATransform::new(*move_char, dfastate_index as usize));
            }
        }
        index = index + 1;
    }
    dfa
}

fn main() {
    println!("输入正则表达式(不支持+,?,-运算符):");
    let mut regx = String::new();
    stdin().read_line(&mut regx).expect("input error.");
    let regx = regx.trim();
    let regx_char_vec: Vec<RegxChar> = regx_to_suffix(regx.to_string());
    print_regx(&regx_char_vec);
    let (nfastate_vec, nfa) = convert_regx_nfa(regx_char_vec);
    print_nfa(&nfa, &nfastate_vec);
    let dfa = convert_nfa_dfa(&nfastate_vec, &nfa);
    dfa.print_dfa();
    println!("输入待检查字符串:");
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).expect("input error.");
    let input_str = input_str.trim();
    println!("{}", dfa.check_string(&input_str));
}
