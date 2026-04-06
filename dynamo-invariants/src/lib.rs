#![forbid(unsafe_code)]

//! Clean invariants interface for Lemma 4.1.

/// Marker returned by Lemma 4.1 implementations that support
/// extraction-soundness style reasoning.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ExtractionSoundnessMarker;

/// A minimal interface for checking the consistency conditions used by
/// Lemma 4.1 over a trace of field elements.
pub trait Lemma41<F> {
    /// Returns `true` when the provided field element satisfies the base
    /// consistency condition of the invariant.
    fn is_consistent(&self, field_element: &F) -> bool;

    /// Returns `true` when the transition from `current` to `next`
    /// preserves the invariant.
    fn step(&self, current: &F, next: &F) -> bool;

    /// Checks the invariant across an entire execution trace.
    fn check_trace(&self, trace: &[F]) -> bool {
        match trace {
            [] => true,
            [field_element] => self.is_consistent(field_element),
            [first, rest @ ..] => {
                if !self.is_consistent(first) {
                    return false;
                }

                let mut current = first;
                for next in rest {
                    if !self.step(current, next) {
                        return false;
                    }
                    current = next;
                }

                true
            }
        }
    }

    /// Returns a marker indicating that the implementation participates
    /// in extraction-soundness style arguments.
    #[inline(always)]
    fn extraction_soundness_marker(&self) -> ExtractionSoundnessMarker {
        ExtractionSoundnessMarker
    }
}

/// Convenience helper for validating a trace against a Lemma 4.1 invariant.
#[inline(always)]
pub fn lemma_4_1_holds<F, L>(invariant: &L, trace: &[F]) -> bool
where
    L: Lemma41<F>,
{
    invariant.check_trace(trace)
}
