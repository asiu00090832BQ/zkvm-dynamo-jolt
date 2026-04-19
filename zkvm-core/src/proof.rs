// proof.rs
use crate::decoder::Instruction;
use crate::vm::{StepOutcome, StepStatus, VM};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofArtifact { pub step_index: u64, pub pc_before: u32, pub pc_after: u32, pub instruction: Instruction, pub status: StepStatus, pub cycles_before: u64, pub cycles_after: u64 }
#[derive(Clone, Debug, Default)]
pub struct ProofPipeline { pub artifacts: Vec<ProofArtifact> }
impl ProofPipeline {
    pub fn new() -> Self { Self::default() }
    pub fn record_step(&mut self, i: u64, o: &StepOutcome) { self.artifacts.push(ProofArtifact { step_index: i, pc_before: o.pc_before, pc_after: o.pc_after, instruction: o.instruction, status: o.status.clone(), cycles_before: o.cycles_before, cycles_after: o.cycles_after }) }
    pub fn run_with_proof(&mut self, vm: &mut VM, max: u64) -> (Option<StepStatus>, u64) { let mut s = None; let mut c = 0; for i in 0..max { let o = vm.step(); c += 1; s = Some(o.status.clone()); self.record_step(i, &o); if !matches!(o.status, StepStatus::Continued) { break } } (s, c) }
    pub fn print_artifacts(&self) { for a in &self.artifacts { println!("step={:06} pc_before=0x{:08x} pc_after=0x{:08x} instr={:?} status={:?}", a.step_index, a.pc_before, a.pc_after, a.instruction, a.status) } }
}
pub fn lemma_6_1_1_single_step_determinism(vm: &VM) -> bool { let mut v1 = vm.clone(); let mut v2 = vm.clone(); v1.step() == v2.step() && v1 == v2 }