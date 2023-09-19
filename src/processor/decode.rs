use std::fmt::Display;

use bitmatch::bitmatch;
use bitvec::{field::BitField, prelude::Lsb0, view::BitView};

use super::{register::XRegisters, ProcessorError, ProcessorErrorTrait};

pub enum DecodeErrorType {
    NotMatchOpcode,
}

pub struct DecodeError {
    error_type: DecodeErrorType,
}

impl DecodeError {
    fn new(error_type: DecodeErrorType) -> Box<Self> {
        Box::new(DecodeError { error_type })
    }
}

impl ProcessorErrorTrait for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            DecodeErrorType::NotMatchOpcode => write!(f, "Not match opcode."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    LW,
    SW,
}

#[derive(Debug, Clone, Copy)]
pub struct DecodeResult {
    pub opcode: Opcode,

    pub rs1_data: u32,
    pub rs2_data: u32,
    pub wb_addr: u32,

    pub imm_i: u32,
    pub imm_i_sext: i32,
}

pub struct Decode();

impl Decode {
    pub fn decode(&self, inst: u32, xregs: &XRegisters) -> Result<DecodeResult, ProcessorError> {
        let inst_slice = inst.view_bits::<Lsb0>();
        let rs1_addr = inst_slice[15..=19].load::<u32>(); // 15-19bit
        let rs2_addr = inst_slice[20..=24].load::<u32>(); // 20-24bit
        let wb_addr = inst_slice[7..=11].load::<u32>(); // 7-11bit

        let rs1_data = xregs.read(rs1_addr);
        let rs2_data = xregs.read(rs2_addr);

        let imm_i = inst_slice[20..=31].load::<u32>();
        let imm_i_sext = inst_slice[20..=31].load::<i32>();

        println!("{}", imm_i_sext);

        if let Some(opcode) = self.match_opcode(inst) {
            println!("Decode: opcode \x1b[38;5;2m{:?}\x1b[m", opcode);
            println!(
                "        rs1_addr 0b{:0>5b}({}), rs2_addr 0b{:0>5b}({}), wb_addr 0b{:0>5b}({})",
                rs1_addr, rs1_addr, rs2_addr, rs2_addr, wb_addr, wb_addr
            );

            println!(
                "        rs1_data 0b{:0>5b}({}), rs2_data 0b{:0>5b}({})",
                rs1_data, rs1_data, rs2_data, rs2_data
            );

            Ok(DecodeResult {
                opcode,
                rs1_data,
                rs2_data,
                wb_addr,
                imm_i,
                imm_i_sext,
            })
        } else {
            Err(DecodeError::new(DecodeErrorType::NotMatchOpcode))
        }
    }

    #[bitmatch]
    fn match_opcode(&self, inst: u32) -> Option<Opcode> {
        #[bitmatch]
        match inst {
            "?????????????????010?????0000011" => Some(Opcode::LW),
            "?????????????????010?????0100011" => Some(Opcode::SW),
            _ => None,
            // None => Err(DecodeError::new(DecodeErrorType::NotMatchOpcode)),
        }
    }
}

/*
pub fn sign_extension(value: u32, n_top: u32) -> u32 {
    // 符号を取得
    let sign = (value >> n_top) % 2;

    // 符号ビットを除外する
    let mut value = value % 2u32.pow(n_top);

    // 符号がマイナス
    if sign == 1 {
        // 符号拡張
        value = u32::MAX - 2u32.pow(n_top) + value + 1
    }

    value
}
*/
