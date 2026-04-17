pub use rv32im_decoder::{
    decode::m_extension::decode_m_extension,
    error::ZkvmError,
    isa::opcode::{
        DecodedInstruction, InstructionWord, MFunct3, MInstruction, MInstructionKind, Opcode,
        RegisterTriple, RV32M_FUNCT7,
    },
    verify::lemma_6_1_1::{decompose_u32_to_limbs_16, verify_lemma_6_1_1, Limb16Product},
    zkvm::{ZkvmConfig, ZkvmDecoder},
};
