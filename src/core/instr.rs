pub type Reg = u8;
pub type Imm = i32;
pub type MemAddr = u32;

// Subset of the RISC-V ISA
#[derive(Debug)]
pub enum Instr {
    // R-Type Instructions
    Add { rd: Reg, rs1: Reg, rs2: Reg },
    Sub { rd: Reg, rs1: Reg, rs2: Reg },
    And { rd: Reg, rs1: Reg, rs2: Reg },
    Or { rd: Reg, rs1: Reg, rs2: Reg },
    Xor { rd: Reg, rs1: Reg, rs2: Reg },

    // I-Type Instructions
    Addi { rd: Reg, rs1: Reg, imm: Imm },
    // S-Type Instructions

    // B-Type Instructions

    // J-Type Instructions

    // U-Type Instructions
}
