use crate::{decoder::MulDivOp, ZkvmError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limb16 { pub low: u16, pub high: u16 }
impl Limb16 {
    pub fn from_u32(v: u32) -> Self { Self { low: v as u16, high: (v >> 16) as u16 } }
    pub fn recompose(self) -> u32 { (self.low as u32) | ((self.high as u32) << 16) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lemma611Witness {
    pub lhs: u32, pub rhs: u32,
    pub p00: u32, pub p01: u32, pub p10: u32, pub p11: u32,
    pub combined: u64,
}

impl Lemma611Witness {
    pub fn new(lhs: u32, rhs: u32) -> Self {
        let l = Limb16::from_u32(lhs); let r = Limb16::from_u32(rhs);
        let p00 = (l.low as u32) * (r.low as u32);
        let p01 = (l.low as u32) * (r.high as u32);
        let p10 = (l.high as u32) * (r.low as u32);
        let p11 = (l.high as u32) * (r.high as u32);
        let combined = (p00 as u64) + (((p01 as u64) + (p10 as u64)) << 16) + ((p11 as u64) << 32);
        Self { lhs, rhs, p00, p01, p10, p11, combined }
    }
    pub fn verify(&self) -> bool { self.combined == (self.lhs as u64) * (self.rhs as u64) }
}

pub fn lemma_6_1_1_witness(l: u32, r: u32) -> Lemma611Witness { Lemma611Witness::new(l, r) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulProofArtifact { pub op: MulDivOp, pub lhs: u32, pub rhs: u32, pub witness: Lemma611Witness, pub result: u32 }

impl MulProofArtifact {
    pub fn new(op: MulDivOp, lhs: u32, rhs: u32) -> Result<Self, ZkvmError> {
        let w = Lemma611Witness::new(lhs, rhs);
        let res = match op {
            MulDivOp::Mul => w.combined as u32,
            MulDivOp::Mulhu => (w.combined >> 32) as u32,
            MulDivOp::Mulh => ((lhs as i32 as i64).wrapping_mul(rhs as i32 as i64) >> 32) as u32,
            MulDivOp::Mulhsu => ((lhs as i32 as i64).wrapping_mul(rhs as u64 as i64) >> 32) as u32,
            _ => return Err(ZkvmError::Trap),
        };
        Ok(Self { op, lhs, rhs, witness: w, result: res })
    }
    pub fn verify(&self) -> bool { self.witness.verify() }
}

pub fn rv32m_mul_artifact(op: MulDivOp, lhs: u32, rhs: u32) -> Result<MulProofArtifact, ZkvmError> { MulProofArtifact::new(op, lhs, rhs) }
pub fn rv32m_mul_result(op: MulDivOp, lhs: u32, rhs: u32) -> Result<(u32, Lemma611Witness), ZkvmError> {
    let art = MulProofArtifact::new(op, lhs, rhs)?;
    Ok((art.result, art.witness))
}

#[derive(Debug, Clone, Default)]
pub struct ProofTrace { pub artifacts: Vec<MulProofArtifact> }
impl ProofTrace {
    pub fn new() -> Self { Self { artifacts: vec![] } }
    pub fn push(&mut self, a: MulProofArtifact) -> Result<(), ZkvmError> {
        if !a.verify() { return Err(ZkvmError::Trap); }
        self.artifacts.push(a); Ok(())
    }
}
