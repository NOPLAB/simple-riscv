use std::fmt::Display;

use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::view::BitView;

use bitpattern::bitpattern;

use super::register::XRegisters;
use super::{ProcessorError, ProcessorErrorTrait};

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

#[derive(Debug)]
pub enum Opcode {
    LW,
    SW,
    None,
}

pub struct DecodeResult {
    pub opcode: Opcode,

    pub rs1: u32,
    pub rs2: u32,
    pub wb_addr: u32,

    pub imm_i: u32,
    pub imm_i_sext: u32,
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
        let imm_i_sext = sign_extension(imm_i, 11);

        let opcode = if bitpattern!("?????????????????010?????0000011", inst) == Some(()) {
            Opcode::LW
        } else if bitpattern!("?????????????????010?????0100011", inst) == Some(()) {
            Opcode::SW
        } else {
            Opcode::None
        };

        println!(
            "Decode: opcode {:?}, rs2_addr 0b{:0>5b}, rs1_addr 0b{:0>5b}, wb_addr 0b{:0>5b}",
            opcode, rs2_addr, rs1_addr, wb_addr
        );

        Ok(DecodeResult {
            opcode,
            rs1: rs1_data,
            rs2: rs2_data,
            wb_addr,
            imm_i,
            imm_i_sext,
        })
    }
}

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
