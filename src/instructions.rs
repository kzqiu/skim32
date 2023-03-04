use std::collections::HashMap;

lazy_static! {
    pub static ref R_FORMAT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("", "");
        m
    };
    pub static ref I_FORMAT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("", "");
        m
    };
    pub static ref JUMP_FORMAT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("", "");
        m
    };
}

// enum InstrType {
//     Register,
//     MemAccess,
//     Jump,
// }

// fn op_code(instr: &str) -> String {
//     let s = String::from(" ");
//     s
// }

pub fn process_instruction(instr: &str) -> String {
    let mut out = String::new();
    let cmds: Vec<&str> = instr.split_whitespace().collect();

    out.push_str(cmds[0]);

    out
}
