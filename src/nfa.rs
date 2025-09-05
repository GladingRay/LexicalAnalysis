use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
/*
    构造NFA部分
*/
pub enum TransformChar {
    NormalChar(char),
    VoidChar,
}

impl TransformChar {
    pub fn from(regx_char: char) -> TransformChar {
        if regx_char == '$' {
            TransformChar::VoidChar
        } else {
            TransformChar::NormalChar(regx_char)
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            TransformChar::VoidChar => '$',
            TransformChar::NormalChar(c) => *c,
        }
    }
}

pub struct NFATransform {
    pub transform: TransformChar,
    pub dest: usize,
}

impl NFATransform {
    pub fn new(trans_: TransformChar, dest_: usize) -> NFATransform {
        NFATransform {
            transform: trans_,
            dest: dest_,
        }
    }
    pub fn new_void(dest_: usize) -> NFATransform {
        NFATransform {
            transform: TransformChar::VoidChar,
            dest: dest_,
        }
    }
}

pub struct NFAState {
    _name: usize,
    transforms: Vec<NFATransform>,
}

impl NFAState {
    pub fn new(_name: usize) -> NFAState {
        NFAState {
            _name,
            transforms: Vec::new(),
        }
    }

    pub fn add_transform(&mut self, transform: NFATransform) {
        self.transforms.push(transform);
    }

    pub fn get_transforms(&self) -> &Vec<NFATransform> {
        &self.transforms
    }
}

pub struct NFA {
    start: usize,
    end: usize,
}

impl NFA {
    pub fn new(ts: usize, te: usize) -> NFA {
        NFA { start: ts, end: te }
    }
    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_end(&self) -> usize {
        self.end
    }
}

pub fn get_void_closure(
    nfastate_vec: &Vec<NFAState>,
    now: usize,
    end: usize,
) -> (HashSet<usize>, bool) {
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
        } else {
            0
        };
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

pub fn get_set_void_closure(
    nfastate_vec: &Vec<NFAState>,
    set: &HashSet<usize>,
    end: usize,
) -> (HashSet<usize>, bool) {
    let mut res_closure: HashSet<usize> = HashSet::new();
    let mut res_is_end: bool = false;
    for now in set {
        let (temp_closure, temp_is_end) = get_void_closure(nfastate_vec, *now, end);
        if temp_is_end {
            res_is_end = true;
        }
        res_closure.extend(&temp_closure);
    }
    (res_closure, res_is_end)
}

pub fn get_move_set(
    nfastate_vec: &Vec<NFAState>,
    now_set: &HashSet<usize>,
) -> HashMap<char, HashSet<usize>> {
    let mut move_map: HashMap<char, HashSet<usize>> = HashMap::new();
    for state in now_set {
        for transform in nfastate_vec[*state].get_transforms() {
            if let TransformChar::NormalChar(c) = transform.transform {
                if !move_map.contains_key(&c) {
                    let mut move_set: HashSet<usize> = HashSet::new();
                    move_set.insert(transform.dest);
                    move_map.insert(c, move_set);
                } else {
                    if let Some(temp_set) = move_map.get_mut(&c) {
                        temp_set.insert(transform.dest);
                    };
                }
            };
        }
    }
    move_map
}

pub fn print_nfa(nfa: &NFA, nfastate_vec: &Vec<NFAState>) {
    let mut flag: Vec<bool> = Vec::new();
    let mut index = 0;
    println!("NFA 部分:");
    println!(
        "this nfa start on {}, end on {}",
        nfa.get_start(),
        nfa.get_end()
    );
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
        } else {
            println!("147 error!");
            nfa.get_start()
        };
        for transform in nfastate_vec[temp_state].get_transforms() {
            println!(
                "{}-{}->{} ",
                temp_state,
                transform.transform.to_char(),
                transform.dest
            );
            if flag[transform.dest] == false {
                queue.push_back(transform.dest);
                flag[transform.dest] = true;
            }
        }
    }
}
