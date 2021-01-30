pub struct Instruction {
    original: u16,
    prefix: u8,
    nnn: u16,
    x: u8,
    y: u8,
    kk: u8,
    suffix_4: u8,
    suffix_8: u8,
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        Self::new(instruction)
    }
}

impl Instruction {
    pub fn new(instruction: u16) -> Self {
        Instruction {
            original: instruction,
            prefix: (instruction >> 12) as u8,
            nnn: instruction & 0x0FFF,
            x: ((instruction & 0x0F00) >> 8) as u8,
            y: ((instruction & 0x00F0) >> 4) as u8,
            kk: (instruction & 0x00FF) as u8,
            suffix_4: (instruction & 0x000F) as u8,
            suffix_8: (instruction & 0x00FF) as u8,
        }
    }

    pub fn nnn(&self) -> u16 {
        self.nnn
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn kk(&self) -> u8 {
        self.kk
    }

    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    pub fn suffix_4(&self) -> u8 {
        self.suffix_4
    }

    pub fn suffix_8(&self) -> u8 {
        self.suffix_8
    }

    pub fn original(&self) -> u16 {
        self.original
    }
}
