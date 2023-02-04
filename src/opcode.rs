// > 指针加一
// < 指针减一
// + 指针指向的字节的值加一
// - 指针指向的字节的值减一
// . 输出指针指向的单元内容(ASCII码)
// , 输入内容到指针指向的单元(ASCII码)
// [ 如果指针指向的单元值为零，向后跳转到对应的 ] 指令的次一指令处
// ] 如果指针指向的单元值不为零，向前跳转到对应的 [ 指令的次一指令处

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Code {
    // 指令
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHL as u8,
            Opcode::SHR as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::GETCHAR as u8,
            Opcode::PUTCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];

        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();
        for (i, e) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i);
            }
            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty list")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }

        Ok(Code { instrs, jtable })
    }
}
