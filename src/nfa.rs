
/*
    构造NFA部分
*/
pub enum TransformChar {
    NormalChar(char),
    VoidChar
}

impl TransformChar {
    pub fn from(regx_char : char) -> TransformChar {
        if regx_char == '$' {
            TransformChar::VoidChar
        }
        else {
            TransformChar::NormalChar(regx_char)
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            TransformChar::VoidChar => {
                '$'
            },
            TransformChar::NormalChar(c) => {
                *c
            }
        }
    }
}

pub struct NFATransform {
    pub transform : TransformChar,
    pub dest : usize
}

impl NFATransform {
    pub fn new (trans_ : TransformChar, dest_ : usize) -> NFATransform {
        NFATransform {
            transform: trans_,
            dest: dest_
        }
    }
    pub fn new_void(dest_ : usize) -> NFATransform {
        NFATransform {
            transform: TransformChar::VoidChar,
            dest: dest_
        }
    }
}

pub struct NFAState {
    name : usize,
    transforms : Vec<NFATransform>
}

impl NFAState {
    pub fn new (name: usize) -> NFAState {

        NFAState {
            name,
            transforms: Vec::new()
        }
    }

    pub fn add_transform (&mut self, transform: NFATransform) {
        self.transforms.push(transform);
    }

    pub fn get_transforms(&self) -> &Vec<NFATransform> {
        &self.transforms
    }
}

pub struct NFA {
    start : usize,
    end : usize
}

impl NFA {
    pub fn new (ts: usize, te: usize) -> NFA {
        NFA {
            start: ts,
            end : te
        }
    }
    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_end(&self) -> usize {
        self.end
    }
}