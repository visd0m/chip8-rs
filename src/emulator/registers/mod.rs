use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistersError {
    #[error("Stack overflow error")]
    StackOverflow,
    #[error("Empty stack on return")]
    EmptyStack,
    #[error("Invalid register: `{0}`")]
    InvalidRegister(u8),
}

pub struct Registers {
    v_0: u8,
    v_1: u8,
    v_2: u8,
    v_3: u8,
    v_4: u8,
    v_5: u8,
    v_6: u8,
    v_7: u8,
    v_8: u8,
    v_9: u8,
    v_a: u8,
    v_b: u8,
    v_c: u8,
    v_d: u8,
    v_e: u8,
    // used as flag
    v_f: u8,
    // used to store memory addresses
    i: u16,
    // when non 0 automatically decremented at a rate of 60Hz
    dt: u8,
    st: u8,
    // not used by the programs
    pc: u16,
    stack: Vec<u16>,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            v_0: 0,
            v_1: 0,
            v_2: 0,
            v_3: 0,
            v_4: 0,
            v_5: 0,
            v_6: 0,
            v_7: 0,
            v_8: 0,
            v_9: 0,
            v_a: 0,
            v_b: 0,
            v_c: 0,
            v_d: 0,
            v_e: 0,
            v_f: 0,
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            stack: vec![],
        }
    }
}

impl Registers {
    pub fn register(&self, x: u8) -> Result<u8, RegistersError> {
        match x {
            0x00 => Ok(self.v_0),
            0x01 => Ok(self.v_1),
            0x02 => Ok(self.v_2),
            0x03 => Ok(self.v_3),
            0x04 => Ok(self.v_4),
            0x05 => Ok(self.v_5),
            0x06 => Ok(self.v_6),
            0x07 => Ok(self.v_7),
            0x08 => Ok(self.v_8),
            0x09 => Ok(self.v_9),
            0x0a => Ok(self.v_a),
            0x0b => Ok(self.v_b),
            0x0c => Ok(self.v_c),
            0x0d => Ok(self.v_d),
            0x0e => Ok(self.v_e),
            0x0f => Ok(self.v_f),
            _ => Err(RegistersError::InvalidRegister(x)),
        }
    }
    pub fn i(&self) -> u16 {
        self.i
    }
    pub fn dt(&self) -> u8 {
        self.dt
    }
    pub fn st(&self) -> u8 {
        self.st
    }
    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_register(&mut self, x: u8, byte: u8) -> Result<(), RegistersError> {
        match x {
            0x00 => {
                self.v_0 = byte;
                Ok(())
            }
            0x01 => {
                self.v_1 = byte;
                Ok(())
            }
            0x02 => {
                self.v_2 = byte;
                Ok(())
            }
            0x03 => {
                self.v_3 = byte;
                Ok(())
            }
            0x04 => {
                self.v_4 = byte;
                Ok(())
            }
            0x05 => {
                self.v_5 = byte;
                Ok(())
            }
            0x06 => {
                self.v_6 = byte;
                Ok(())
            }
            0x07 => {
                self.v_7 = byte;
                Ok(())
            }
            0x08 => {
                self.v_8 = byte;
                Ok(())
            }
            0x09 => {
                self.v_9 = byte;
                Ok(())
            }
            0x0a => {
                self.v_a = byte;
                Ok(())
            }
            0x0b => {
                self.v_b = byte;
                Ok(())
            }
            0x0c => {
                self.v_c = byte;
                Ok(())
            }
            0x0d => {
                self.v_d = byte;
                Ok(())
            }
            0x0e => {
                self.v_e = byte;
                Ok(())
            }
            0x0f => {
                self.v_f = byte;
                Ok(())
            }

            _ => Err(RegistersError::InvalidRegister(x)),
        }
    }

    pub fn set_v_f(&mut self, v_f: u8) {
        self.v_f = v_f;
    }
    pub fn set_i(&mut self, i: u16) {
        self.i = i;
    }
    pub fn set_dt(&mut self, display: u8) {
        self.dt = display;
    }
    pub fn set_st(&mut self, sound: u8) {
        self.st = sound;
    }
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn pop_stack(&mut self) -> Result<(), RegistersError> {
        let addr = self.stack.last().ok_or(RegistersError::EmptyStack)?;
        self.pc = *addr;
        self.stack.pop();
        Ok(())
    }

    pub fn inc_pc_by(&mut self, value: u16) {
        self.pc += value
    }

    pub fn decrement_st(&mut self) {
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn decrement_dt(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn push_stack(&mut self, addr: u16) -> Result<(), RegistersError> {
        if self.stack.len() == 16 {
            return Err(RegistersError::StackOverflow);
        }
        self.stack.push(self.pc);
        self.pc = addr;

        Ok(())
    }
}
