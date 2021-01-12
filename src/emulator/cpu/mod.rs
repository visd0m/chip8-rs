use crate::emulator::audio::{Audio, AudioError};
use crate::emulator::cpu::instruction::Instruction;
use crate::emulator::display::{Display, HEIGHT, WIDTH};
use crate::emulator::keyboard::{Keyboard, KeyboardError};
use crate::emulator::memory::Memory;
use crate::emulator::registers::{Registers, RegistersError};
use rand::rngs::ThreadRng;
use rand::Rng;
use thiserror::Error;

mod instruction;

pub struct Cpu {
    memory: Memory,
    registers: Registers,
    is_waiting_key: bool,
    waiting_key_register: u8,
    rng: ThreadRng,
}

#[derive(Debug, Error)]
pub enum CpuError {
    #[error("Unhandled instruction: {:x}", .0)]
    UnhandledInstruction(u16),
    #[error(transparent)]
    RegistersError(#[from] RegistersError),
    #[error(transparent)]
    KeyboardError(#[from] KeyboardError),
    #[error(transparent)]
    SoundError(#[from] AudioError),
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: Default::default(),
            is_waiting_key: false,
            waiting_key_register: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn cycle(
        &mut self,
        display: &mut Display,
        sound: &mut Audio,
        keyboard: Keyboard,
    ) -> Result<(), CpuError> {
        if self.is_waiting_key {
            if let Some(key) = keyboard.get_key_pressed() {
                self.registers
                    .set_register(self.waiting_key_register, key)?;
                self.is_waiting_key = false;
            }

            return Ok(());
        }

        if self.registers.st() > 0 {
            sound.beep()?;
        }

        if self.registers.st() == 0 {
            sound.stop_beep()?;
        }

        self.registers.decrement_st();
        self.registers.decrement_dt();

        let instruction = self.memory.read_16(self.registers.pc());
        self.registers.inc_pc_by(2);

        let i: Instruction = instruction.into();

        match i.prefix() {
            0x0 => self.x0nnn(i, display),
            0x1 => {
                self.jp(i.nnn());
                Ok(())
            }
            0x2 => Ok(self.call(i.nnn())?),
            0x3 => Ok(self.se(i.x(), i.kk())?),
            0x4 => Ok(self.sne(i.x(), i.kk())?),
            0x5 => Ok(self.se_reg(i.x(), i.y())?),
            0x6 => Ok(self.ld(i.x(), i.kk())?),
            0x7 => Ok(self.add(i.x(), i.kk())?),
            0x8 => self.x8nnn(i),
            0x9 => Ok(self.sne_reg(i.x(), i.y())?),
            0xa => {
                self.ldi(i.nnn());
                Ok(())
            }
            0xb => Ok(self.jp_0(i.nnn())?),
            0xc => Ok(self.rnd(i.x(), i.kk())?),
            0xd => Ok(self.draw(i.x(), i.y(), i.suffix_4(), display)?),
            0xe => self.xennn(i, keyboard),
            0xf => self.xfnnn(i),
            _ => Err(CpuError::UnhandledInstruction(instruction)),
        }?;

        Ok(())
    }

    pub fn x0nnn(
        &mut self,
        instruction: Instruction,
        display: &mut Display,
    ) -> Result<(), CpuError> {
        if instruction.original() == 0x00e0 {
            self.cls(display);
            return Ok(());
        }

        if instruction.original() == 0x00ee {
            return Ok(self.ret()?);
        }

        Err(CpuError::UnhandledInstruction(instruction.original()))
    }

    pub fn x8nnn(&mut self, instruction: Instruction) -> Result<(), CpuError> {
        match instruction.suffix_4() {
            0x0 => Ok(self.ld_reg(instruction.x(), instruction.y())?),
            0x1 => Ok(self.or(instruction.x(), instruction.y())?),
            0x2 => Ok(self.and(instruction.x(), instruction.y())?),
            0x3 => Ok(self.xor(instruction.x(), instruction.y())?),
            0x4 => Ok(self.add_reg(instruction.x(), instruction.y())?),
            0x5 => Ok(self.sub(instruction.x(), instruction.y())?),
            0x6 => Ok(self.shr(instruction.x())?),
            0x7 => Ok(self.subn(instruction.x(), instruction.y())?),
            0xe => Ok(self.shl(instruction.x())?),
            _ => Err(CpuError::UnhandledInstruction(instruction.original())),
        }
    }

    pub fn xennn(&mut self, instruction: Instruction, keyboard: Keyboard) -> Result<(), CpuError> {
        match instruction.suffix_8() {
            0x9e => Ok(self.skp(instruction.x(), keyboard)?),
            0xa1 => Ok(self.sknp(instruction.x(), keyboard)?),
            _ => Err(CpuError::UnhandledInstruction(instruction.original())),
        }
    }

    pub fn xfnnn(&mut self, instruction: Instruction) -> Result<(), CpuError> {
        match instruction.suffix_8() {
            0x07 => Ok(self.ld_from_dt(instruction.x())?),
            0x0a => Ok(self.ld_k(instruction.x())?),
            0x15 => Ok(self.ld_into_dt(instruction.x())?),
            0x18 => Ok(self.ld_into_st(instruction.x())?),
            0x1e => Ok(self.add_i(instruction.x())?),
            0x29 => Ok(self.ld_f(instruction.x())?),
            0x33 => Ok(self.ld_b(instruction.x())?),
            0x55 => Ok(self.ld_batch_into(instruction.x())?),
            0x65 => Ok(self.ld_batch_from(instruction.x())?),
            _ => Err(CpuError::UnhandledInstruction(instruction.original())),
        }
    }

    pub fn cls(&mut self, display: &mut Display) {
        display.clear()
    }

    pub fn ret(&mut self) -> Result<(), CpuError> {
        self.registers.pop_stack()?;
        Ok(())
    }

    pub fn jp(&mut self, addr: u16) {
        self.registers.set_pc(addr)
    }

    pub fn call(&mut self, addr: u16) -> Result<(), RegistersError> {
        self.registers.push_stack(addr)
    }

    pub fn se(&mut self, v_x: u8, byte: u8) -> Result<(), RegistersError> {
        let register_value = self.registers.register(v_x)?;
        if register_value == byte {
            self.registers.inc_pc_by(2)
        }

        Ok(())
    }

    pub fn sne(&mut self, v_x: u8, byte: u8) -> Result<(), RegistersError> {
        let register_value = self.registers.register(v_x)?;
        if register_value != byte {
            self.registers.inc_pc_by(2)
        }

        Ok(())
    }

    pub fn se_reg(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;
        if register_1_value == register_2_value {
            self.registers.inc_pc_by(2)
        }

        Ok(())
    }

    pub fn ld(&mut self, v_x: u8, byte: u8) -> Result<(), RegistersError> {
        self.registers.set_register(v_x, byte)
    }

    pub fn ld_reg(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_2_value = self.registers.register(v_y)?;
        self.registers.set_register(v_x, register_2_value)
    }

    pub fn add(&mut self, v_x: u8, byte: u8) -> Result<(), RegistersError> {
        let register_value = self.registers.register(v_x)?;
        self.registers
            .set_register(v_x, register_value.overflowing_add(byte).0)
    }

    pub fn add_reg(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        let (sum, overflow) = register_1_value.overflowing_add(register_2_value);
        if overflow {
            self.registers.set_v_f(1)
        } else {
            self.registers.set_v_f(0)
        }

        self.registers.set_register(v_x, sum)
    }

    pub fn or(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        self.registers
            .set_register(v_x, register_1_value | register_2_value)
    }

    pub fn and(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        self.registers
            .set_register(v_x, register_1_value & register_2_value)
    }

    pub fn xor(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        self.registers
            .set_register(v_x, register_1_value ^ register_2_value)
    }

    pub fn sub(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        if register_1_value > register_2_value {
            self.registers.set_v_f(1)
        } else {
            self.registers.set_v_f(0)
        }

        self.registers
            .set_register(v_x, register_1_value.overflowing_sub(register_2_value).0)
    }

    pub fn shr(&mut self, v_x: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        self.registers.set_v_f((register_1_value & 0x1) as u8);
        self.registers.set_register(v_x, register_1_value / 2)
    }

    pub fn subn(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;

        if register_1_value < register_2_value {
            self.registers.set_v_f(1)
        } else {
            self.registers.set_v_f(0)
        }

        self.registers
            .set_register(v_x, register_1_value - register_2_value)
    }

    pub fn shl(&mut self, v_x: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        self.registers
            .set_v_f(((register_1_value & 0xA0) >> 7) as u8);
        self.registers
            .set_register(v_x, register_1_value.overflowing_mul(2).0)
    }

    pub fn sne_reg(&mut self, v_x: u8, v_y: u8) -> Result<(), RegistersError> {
        let register_1_value = self.registers.register(v_x)?;
        let register_2_value = self.registers.register(v_y)?;
        if register_1_value != register_2_value {
            self.registers.inc_pc_by(2)
        }

        Ok(())
    }

    pub fn ldi(&mut self, addr: u16) {
        self.registers.set_i(addr)
    }

    pub fn jp_0(&mut self, addr: u16) -> Result<(), RegistersError> {
        self.registers
            .set_pc(addr + self.registers.register(0)? as u16);
        Ok(())
    }

    pub fn rnd(&mut self, v_x: u8, byte: u8) -> Result<(), RegistersError> {
        let random: u8 = self.rng.gen_range(0..255);
        self.registers.set_register(v_x, random & byte)
    }

    pub fn draw(
        &mut self,
        v_x: u8,
        v_y: u8,
        nibble: u8,
        display: &mut Display,
    ) -> Result<(), CpuError> {
        let x = self.registers.register(v_x)?;
        let y = self.registers.register(v_y)?;

        self.registers.set_v_f(0);
        for row in 0..nibble {
            let byte = self.memory.read_8(self.registers.i() + row as u16);

            for column in 0..8 {
                let x = (x + column) % WIDTH as u8;
                let y = (y + row) % HEIGHT as u8;

                let old_value = display.pixel(x as usize, y as usize);
                let to_set: u32 = if (((byte as usize) >> (7 - column)) & 0x1) > 0 {
                    0xffffffff
                } else {
                    0x00000000
                };

                display.set_pixel(x as usize, y as usize, to_set);

                if old_value && !display.pixel(x as usize, y as usize) {
                    self.registers.set_v_f(1);
                }
            }
        }

        Ok(())
    }

    pub fn skp(&mut self, v_x: u8, keyboard: Keyboard) -> Result<(), CpuError> {
        if keyboard.is_key_pressed(self.registers.register(v_x)?) {
            self.registers.inc_pc_by(2);
        }

        Ok(())
    }

    pub fn sknp(&mut self, v_x: u8, keyboard: Keyboard) -> Result<(), CpuError> {
        if !keyboard.is_key_pressed(self.registers.register(v_x)?) {
            self.registers.inc_pc_by(2);
        }

        Ok(())
    }

    pub fn ld_from_dt(&mut self, v_x: u8) -> Result<(), RegistersError> {
        self.registers.set_register(v_x, self.registers.dt())
    }

    pub fn ld_k(&mut self, v_x: u8) -> Result<(), RegistersError> {
        self.is_waiting_key = true;
        self.waiting_key_register = v_x;
        Ok(())
    }

    pub fn ld_into_dt(&mut self, v_x: u8) -> Result<(), RegistersError> {
        self.registers.set_dt(self.registers.register(v_x)?);
        Ok(())
    }

    pub fn ld_into_st(&mut self, v_x: u8) -> Result<(), RegistersError> {
        self.registers.set_st(self.registers.register(v_x)?);
        Ok(())
    }

    pub fn add_i(&mut self, v_x: u8) -> Result<(), RegistersError> {
        self.registers.set_i(
            self.registers
                .i()
                .overflowing_add(self.registers.register(v_x)? as u16)
                .0,
        );
        Ok(())
    }

    pub fn ld_f(&mut self, v_x: u8) -> Result<(), CpuError> {
        let font = self.registers.register(v_x)?;
        self.registers.set_i(self.memory.get_font_address(font));
        Ok(())
    }

    pub fn ld_b(&mut self, v_x: u8) -> Result<(), RegistersError> {
        let value = self.registers.register(v_x)?;

        let units = value % 10;
        let tens = (value / 10) % 10;
        let hundreds = (value / 100) % 10;

        self.memory.write_8(self.registers.i(), hundreds);
        self.memory.write_8(self.registers.i() + 1, tens);
        self.memory.write_8(self.registers.i() + 2, units);

        Ok(())
    }

    pub fn ld_batch_into(&mut self, v_x: u8) -> Result<(), CpuError> {
        for index in 0..(v_x + 1) {
            let to_write = self.registers.register(index)?;
            self.memory
                .write_8(self.registers.i() + index as u16, to_write);
        }

        Ok(())
    }

    pub fn ld_batch_from(&mut self, v_x: u8) -> Result<(), CpuError> {
        for index in 0..(v_x + 1) {
            let to_load = self.memory.read_8(self.registers.i() + index as u16);

            self.registers.set_register(index, to_load)?;
        }

        Ok(())
    }
}
