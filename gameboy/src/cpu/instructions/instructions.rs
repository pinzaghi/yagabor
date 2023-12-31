use std::io::Error;

use crate::{cpu::cpu::{MachineCycles, CPU}, mmu::MMU, gameboy::GameBoy, io::interrupts::Interrupts};

use super::decode::*;

impl Instruction {

    pub(crate) fn execute(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        match self.clone() {
            Instruction::CALL(test)             => self.call(gb, test),
            Instruction::RET(test)              => self.ret(gb, test),
            Instruction::JP(test)               => self.jump(gb, test),
            Instruction::JR(test)               => self.jump_relative(gb, test),
            Instruction::JPHL                             => self.jump_hl(gb),
            Instruction::NOP                              => self.nop(gb),
            Instruction::HALT                             => self.halt(gb),
            Instruction::SCF                              => self.scf(gb),
            Instruction::CCF                              => self.ccf(gb),
            Instruction::CPL                              => self.cpl(gb),
            Instruction::ADD(target)     => self.add(gb, target),
            Instruction::ADC(target)     => self.adc(gb, target),
            Instruction::INC(target)   => self.inc(gb, target),
            Instruction::DEC(target)   => self.dec(gb, target),
            Instruction::ADD16(target)      => self.add16(gb, target),
            Instruction::INC16(target)      => self.inc16(gb, target),
            Instruction::DEC16(target)      => self.dec16(gb, target),
            Instruction::ADDSPS8                          => self.addsps8(gb),
            Instruction::SUB(target)     => self.sub(gb, target),
            Instruction::SBC(target)     => self.sbc(gb, target),
            Instruction::AND(target)     => self.and(gb, target),
            Instruction::XOR(target)     => self.xor(gb, target),
            Instruction::OR(target)      => self.or(gb, target),
            Instruction::CP(target)      => self.cp(gb, target),
            Instruction::LD(load_type)          => self.load(gb, load_type),
            Instruction::LDHLSPD8                         => self.ldhlspd8(gb),
            Instruction::LDSPHL                           => self.ldsphl(gb),
            Instruction::LDSPA16                          => self.ldspa16(gb),
            Instruction::LDFF(load_type)      => self.ldff(gb, load_type),
            Instruction::PUSH(target)        => self.push(gb, target),
            Instruction::POP(target)         => self.pop(gb, target),
            Instruction::RST(target)           => self.rst(gb, target),
            Instruction::BIT(bit_type)           => self.bit(gb, bit_type),
            Instruction::RETI                             => self.reti(gb),
            Instruction::DAA                              => self.daa(gb),
            Instruction::RL(target)    => self.rl(gb, target),
            Instruction::RLC(target)   => self.rlc(gb, target),
            Instruction::RR(target)    => self.rr(gb, target),
            Instruction::RRC(target)   => self.rrc(gb, target),
            Instruction::RLA                              => self.rla(gb),
            Instruction::RLCA                             => self.rlca(gb),
            Instruction::RRA                              => self.rra(gb),
            Instruction::RRCA                             => self.rrca(gb),
            Instruction::SRA(target)   => self.sra(gb, target),
            Instruction::SLA(target)   => self.sla(gb, target),
            Instruction::SRL(target)   => self.srl(gb, target),
            Instruction::SWAP(target)  => self.swap(gb, target),
            Instruction::EI                               => self.ei(gb),
            Instruction::DI                               => self.di(gb),
            Instruction::RES(target)          => self.res(gb, target),
            Instruction::SET(target)          => self.set(gb, target),
            Instruction::STOP                             => self.nop(gb),
        }
    }    

    fn jump(&self, gb: &mut GameBoy , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(gb, test);
     
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = MMU::read_byte(gb, gb.cpu.pc + 1) as u16;
            let most_significant_byte = MMU::read_byte(gb, gb.cpu.pc + 2) as u16;
            gb.cpu.pc = (most_significant_byte << 8) | least_significant_byte;
    
            Ok(MachineCycles::Four)
        } else {
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            Ok(MachineCycles::Three)
        }
    }
    
    fn jump_relative(&self, gb: &mut GameBoy , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(gb, test);
     
        if should_jump {
            let offset: i8 = MMU::read_next_byte(gb, gb.cpu.pc) as i8;
            gb.cpu.pc = gb.cpu.pc.wrapping_add(2i8.wrapping_add(offset) as u16);
    
            Ok(MachineCycles::Three)
        } else {
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            Ok(MachineCycles::Two) 
        }
    }
    
    fn jump_hl(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.pc = gb.cpu.regs.get_hl();
        Ok(MachineCycles::One)
    }   
        
    fn nop(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn ei(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.ime = true;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn di(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.ime = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn scf(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.regs.flags.carry = true;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn cpl(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.regs.a = !gb.cpu.regs.a; 
        gb.cpu.regs.flags.subtract = true;
        gb.cpu.regs.flags.half_carry = true;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn ccf(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.regs.flags.carry = !gb.cpu.regs.flags.carry;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    // https://forums.nesdev.org/viewtopic.php?t=15944
    fn daa(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        if !gb.cpu.regs.flags.subtract {  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
            if gb.cpu.regs.flags.carry || gb.cpu.regs.a > 0x99 { 
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_add(0x60);
                gb.cpu.regs.flags.carry = true; 
            }
            if gb.cpu.regs.flags.half_carry || (gb.cpu.regs.a & 0x0f) > 0x09 { 
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_add(0x6); 
            }
        } else {  // after a subtraction, only adjust if (half-)carry occurred
            if gb.cpu.regs.flags.carry { 
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_sub(0x60);
            }
            if gb.cpu.regs.flags.half_carry { 
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_sub(0x06);
            }
        }
        // these flags are always updated
        gb.cpu.regs.flags.zero = gb.cpu.regs.a == 0; // the usual z flag
        gb.cpu.regs.flags.half_carry = false; // h flag is always cleared
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }
    
    fn call(&self, gb: &mut GameBoy , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(gb, test);

        if should_jump {
            let jump_addr = MMU::read_next_word(gb, gb.cpu.pc);
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            CPU::push_stack(gb, gb.cpu.pc);
            gb.cpu.pc = jump_addr;
            Ok(MachineCycles::Six)
        } else {
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            Ok(MachineCycles::Three)
        }
    }
    
    fn ret(&self, gb: &mut GameBoy , test: JumpTest) -> Result<MachineCycles, Error> {
        let jump_condition = should_jump(gb, test);
        if jump_condition {
            gb.cpu.pc = CPU::pop_stack(gb, );
            Ok(MachineCycles::Five) 
        } else {
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            Ok(MachineCycles::Two) 
        }
    }
    
    fn reti(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.pc = CPU::pop_stack(gb, );
        gb.cpu.ime = true;
        Ok(MachineCycles::Four)
    }
    
    fn rst(&self, gb: &mut GameBoy , target: BitTarget) -> Result<MachineCycles, Error> {
        CPU::push_stack(gb, gb.cpu.pc.wrapping_add(u16::from(self.size())));
    
        let address: u16 = match target {
            BitTarget::Zero => 0x0000,
            BitTarget::One => 0x0008,
            BitTarget::Two => 0x0010,
            BitTarget::Three => 0x0018,
            BitTarget::Four => 0x0020,
            BitTarget::Five => 0x0028,
            BitTarget::Six => 0x0030,
            BitTarget::Seven => 0x0038,
        };
        gb.cpu.pc = address; 
    
        Ok(MachineCycles::Four)
    }
    
    fn ldhlspd8(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        let value = MMU::read_next_byte(gb, gb.cpu.pc) as i8 as u16;
        let new_value = gb.cpu.sp.wrapping_add(value);
    
        gb.cpu.regs.flags.zero = false;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.carry = (gb.cpu.sp & 0xFF).wrapping_add(value as u16 & 0xFF) > 0xFF;
        gb.cpu.regs.flags.half_carry = (gb.cpu.sp & 0xF) + (value as u16 & 0xF) > 0xF;
    
        gb.cpu.regs.set_hl(new_value);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Three)        
    }
    
    fn ldsphl(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        gb.cpu.sp = gb.cpu.regs.get_hl();
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Two) 
    }
    
    fn ldspa16(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        let address = MMU::read_next_word(gb, gb.cpu.pc);
    
        let lsb = (gb.cpu.sp & 0x00FF) as u8;
        let msb = ((gb.cpu.sp & 0xFF00) >> 8) as u8;

        MMU::write_byte(gb, address, lsb);
        MMU::write_byte(gb, address.wrapping_add(1), msb);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Five) 
    }
    
    fn load(&self, gb: &mut GameBoy , load_type: LoadType) -> Result<MachineCycles, Error> {
         
        match &load_type {
            LoadType::Byte(target, source) => {
                let source_value = match source {
                    RegistersIndDir::A   => gb.cpu.regs.a,
                    RegistersIndDir::B   => gb.cpu.regs.b,
                    RegistersIndDir::C   => gb.cpu.regs.c,
                    RegistersIndDir::D   => gb.cpu.regs.d,
                    RegistersIndDir::E   => gb.cpu.regs.e,
                    RegistersIndDir::H   => gb.cpu.regs.h,
                    RegistersIndDir::L   => gb.cpu.regs.l,
                    RegistersIndDir::D8  => MMU::read_next_byte(gb, gb.cpu.pc),
                    RegistersIndDir::HLI => MMU::read_byte(gb, gb.cpu.regs.get_hl())
                };
                match target {
                    RegistersIndirect::A   => gb.cpu.regs.a = source_value,
                    RegistersIndirect::B   => gb.cpu.regs.b = source_value,
                    RegistersIndirect::C   => gb.cpu.regs.c = source_value,
                    RegistersIndirect::D   => gb.cpu.regs.d = source_value,
                    RegistersIndirect::E   => gb.cpu.regs.e = source_value,
                    RegistersIndirect::H   => gb.cpu.regs.h = source_value,
                    RegistersIndirect::L   => gb.cpu.regs.l = source_value,
                    RegistersIndirect::HLI => MMU::write_byte(gb, gb.cpu.regs.get_hl(), source_value)
                };
    
            },
            LoadType::Word(target) => {
                match target {
                    WordRegister::BC => {
                        gb.cpu.regs.set_bc(MMU::read_next_word(gb, gb.cpu.pc));
                    },
                    WordRegister::DE => {
                        gb.cpu.regs.set_de(MMU::read_next_word(gb, gb.cpu.pc));
                    },
                    WordRegister::HL => {
                        gb.cpu.regs.set_hl(MMU::read_next_word(gb, gb.cpu.pc));
                    },
                    WordRegister::SP => {
                        gb.cpu.sp = MMU::read_next_word(gb, gb.cpu.pc);
                    }
                }
            },
            LoadType::AFromIndirect(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = gb.cpu.regs.get_bc();
                        gb.cpu.regs.a = MMU::read_byte(gb, addr);
                    },
                    LoadIndirectSource::DE => {
                        let addr = gb.cpu.regs.get_de();
                        gb.cpu.regs.a = MMU::read_byte(gb, addr);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = gb.cpu.regs.get_hl();
                        gb.cpu.regs.a = MMU::read_byte(gb, addr);
                        let new_value = gb.cpu.regs.get_hl().wrapping_add(1);
                        gb.cpu.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = gb.cpu.regs.get_hl();
                        gb.cpu.regs.a = MMU::read_byte(gb, addr);
                        let new_value = gb.cpu.regs.get_hl().wrapping_sub(1);
                        gb.cpu.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::IndirectFromA(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = gb.cpu.regs.get_bc();
                        MMU::write_byte(gb, addr, gb.cpu.regs.a);
                    },
                    LoadIndirectSource::DE => {
                        let addr = gb.cpu.regs.get_de();
                        MMU::write_byte(gb, addr, gb.cpu.regs.a);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = gb.cpu.regs.get_hl();
                        MMU::write_byte(gb, addr, gb.cpu.regs.a);
                        let new_value = gb.cpu.regs.get_hl().wrapping_add(1);
                        gb.cpu.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = gb.cpu.regs.get_hl();
                        MMU::write_byte(gb, addr, gb.cpu.regs.a);
                        let new_value = gb.cpu.regs.get_hl().wrapping_sub(1);
                        gb.cpu.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::AFromDirect => {
                let address = MMU::read_next_word(gb, gb.cpu.pc);
                gb.cpu.regs.a = MMU::read_byte(gb, address);
            },
            LoadType::DirectFromA => {
                MMU::write_byte(gb, MMU::read_next_word(gb, gb.cpu.pc), gb.cpu.regs.a);
            }
        }

        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
    
        // Result
        match load_type {
            LoadType::Byte(_,RegistersIndDir::HLI) => Ok(MachineCycles::Two) ,
            LoadType::Byte(RegistersIndirect::HLI, _) => Ok(MachineCycles::Two) ,
            LoadType::AFromIndirect(_) => Ok(MachineCycles::Two) ,
            LoadType::IndirectFromA(_) => Ok(MachineCycles::Two) ,
            LoadType::Word(_) => Ok(MachineCycles::Three),
            LoadType::AFromDirect => Ok(MachineCycles::Four),
            LoadType::DirectFromA => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::One),
        }
    }
    
    fn ldff(&self, gb: &mut GameBoy , load_type: LoadFFType) -> Result<MachineCycles, Error> {
        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + gb.cpu.regs.c as u16;       
                MMU::write_byte(gb, addr, gb.cpu.regs.a);
                gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
                Ok(MachineCycles::Two) 
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + gb.cpu.regs.c as u16;        
                gb.cpu.regs.a = MMU::read_byte(gb, addr);
                gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
                Ok(MachineCycles::Two) 
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + MMU::read_next_byte(gb, gb.cpu.pc) as u16;        
                MMU::write_byte(gb, addr, gb.cpu.regs.a);
                gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
                Ok(MachineCycles::Three)
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + MMU::read_next_byte(gb, gb.cpu.pc) as u16;        
                gb.cpu.regs.a = MMU::read_byte(gb, addr);
                gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
                Ok(MachineCycles::Three)
            }
        }
    }
    
    pub(crate) fn push(&self, gb: &mut GameBoy , target: StackTarget) -> Result<MachineCycles, Error> {
        let value = match target {
            StackTarget::BC => gb.cpu.regs.get_bc(),
            StackTarget::DE => gb.cpu.regs.get_de(),
            StackTarget::HL => gb.cpu.regs.get_hl(),
            StackTarget::AF => gb.cpu.regs.get_af(),
        };
        CPU::push_stack(gb, value);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Four)
    }
    
    pub(crate) fn pop(&self, gb: &mut GameBoy , target: StackTarget) -> Result<MachineCycles, Error> {
        let result = CPU::pop_stack(gb, );
        match target {
            StackTarget::BC => gb.cpu.regs.set_bc(result),
            StackTarget::DE => gb.cpu.regs.set_de(result),
            StackTarget::HL => gb.cpu.regs.set_hl(result),
            StackTarget::AF => gb.cpu.regs.set_af(result),
        };
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Three)
    }
    
    fn halt(&self, gb: &mut GameBoy ) -> Result<MachineCycles, Error> {
        if Interrupts::some_interrupt_enabled(gb) {
            if !gb.cpu.ime {
                // Halt bug, no PC increment
            }else{
                // We ignore the halting
                gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
            }
        }else{
            gb.cpu.is_halted = true;
            gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        }
        
        Ok(MachineCycles::One)
    }

}

fn should_jump(gb: &GameBoy, test: JumpTest) -> bool {
    match test {
        JumpTest::NotZero => !gb.cpu.regs.flags.zero,
        JumpTest::NotCarry => !gb.cpu.regs.flags.carry,
        JumpTest::Zero => gb.cpu.regs.flags.zero,
        JumpTest::Carry => gb.cpu.regs.flags.carry,
        JumpTest::Always => true
    }
}
