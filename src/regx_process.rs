use std::collections::VecDeque;

pub enum OperatorKind{
    Cat,
    Or,
    Closure
}


pub enum RegxChar{
    NormalChar(char),
    OperatorChar(OperatorKind),
    LeftBracket,
    RightBracket
}

/*
    正则表达式转换部分
*/
pub fn need_push_stack(stack_top: &RegxChar, now_char: &OperatorKind) -> bool {
    if let RegxChar::LeftBracket = stack_top  {
        return true;
    }
    else if let RegxChar::OperatorChar(oc) = stack_top {
        match oc {
            OperatorKind::Cat => {
                match now_char {
                    OperatorKind::Cat => {
                        return false;
                    },
                    OperatorKind::Closure => {
                        return true;
                    },
                    OperatorKind::Or => {
                        return false;
                    }
                }
            },
            OperatorKind::Closure => {
                match now_char {
                    OperatorKind::Cat => {
                        return false;
                    },
                    OperatorKind::Closure => {
                        return false;
                    },
                    OperatorKind::Or => {
                        return false;
                    }
                }
            },
            OperatorKind::Or => {
                match now_char {
                    OperatorKind::Cat => {
                        return true;
                    },
                    OperatorKind::Closure => {
                        return true;
                    },
                    OperatorKind::Or => {
                        return false;
                    }
                }
            }
        }
    }
    false
}

fn is_operator(c: char) -> bool {
    c == '|' || c == '*' || c == '.'
}

fn is_bracket(c: char) -> bool {
    c == '(' || c == ')'
}

fn is_normal_char(c: char) -> bool {
    !is_operator(c) && !is_bracket(c)
}

fn need_insert_cat(c1: char, c2: char) -> bool {
    is_normal_char(c1) && c2 == '(' || c1 == ')' && is_normal_char(c2) || c1 == '*' && is_normal_char(c2) 
        || is_normal_char(c1) && is_normal_char(c2)
}

pub fn make_regx_char (c: char) -> RegxChar {
    match c {
        '(' => RegxChar::LeftBracket,
        ')' => RegxChar::RightBracket,
        '|' => RegxChar::OperatorChar(OperatorKind::Or),
        '*' => RegxChar::OperatorChar(OperatorKind::Closure),
        '.' => RegxChar::OperatorChar(OperatorKind::Cat),
        _  =>  RegxChar::NormalChar(c),
    }
}

pub fn make_regxchar_char (rc: &RegxChar) -> char {
    match rc {
        RegxChar::LeftBracket => '(',
        RegxChar::RightBracket => ')',
        RegxChar::OperatorChar(OperatorKind::Cat) => '.',
        RegxChar::OperatorChar(OperatorKind::Closure) => '*',
        RegxChar::OperatorChar(OperatorKind::Or) => '|',
        RegxChar::NormalChar(res_c) => *res_c,
    }
}

pub fn insert_cat_regx(regx : String) -> Vec<RegxChar> {
    let mut j = true;
    let mut res: Vec<RegxChar > = Vec::new();
    let mut oc:char = '(';
    for c in regx.chars() {
        if j {
            res.push(make_regx_char(c));
            j = false;
            oc = c;
        }
        else {
            if need_insert_cat(oc, c) {
                res.push(make_regx_char('.'));
            }
            res.push(make_regx_char(c));
            oc = c;
        }
    }
    res
}

pub fn regx_to_suffix(regx : String) -> Vec<RegxChar> {
    let mut regx_suffix : Vec<RegxChar> = Vec::new();
    let regx_vec : Vec<RegxChar> = insert_cat_regx(regx);
    let mut stack : VecDeque<RegxChar> = VecDeque::new();
    for regx_char in  regx_vec {
        match regx_char {
            RegxChar::NormalChar(_c) => {
                regx_suffix.push(regx_char);
            },
            RegxChar::LeftBracket => {
                stack.push_back(regx_char);
            },
            RegxChar::RightBracket => {
                loop {
                    if let Some(c) = stack.back() {
                        if let RegxChar::LeftBracket = *c {
                            stack.pop_back();
                            break;
                        }
                        else if let RegxChar::OperatorChar(_nc) = c {
                            let temp = stack.pop_back();
                            if let Some(x) = temp {
                                regx_suffix.push(x);
                            }
                        }
                    }
                    else if let None = stack.back() {
                        break;
                    }
                }  
            },
            RegxChar::OperatorChar(oc) => {
                loop {
                    
                    match stack.back() {
                        Some(c) => {
                            if need_push_stack(c, &oc) {
                                stack.push_back(RegxChar::OperatorChar(oc));
                                break;
                            }
                            else {
                                if let Some(x) = stack.pop_back() {
                                    regx_suffix.push(x);
                                }
                            }
                        }
                        None => {
                            stack.push_back(RegxChar::OperatorChar(oc));
                            break;
                        }
                    }
                }
                
            } 
        }
    }
    loop {
        match stack.pop_back() {
            Some(x) => {
                regx_suffix.push(x);
            },
            None => {
                break;
            }
        }
    }
    regx_suffix
}

pub fn print_regx(regx_char_vec: &Vec<RegxChar>) {
    for elem in regx_char_vec {
        print!("{}", make_regxchar_char(elem));
    }
    println!();
}