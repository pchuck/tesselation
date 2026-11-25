//! Phase-A δ-validity envelope (diss. §3.4.1, brief: parquet deformations).
//!
//! The contact angle on a given substrate determines whether the strapwork
//! strands close. For each of the registered strapwork substrates the set
//! of angles whose strands close over the full window was measured
//! programmatically (1° sweep + 0.1° fine scans + a single-tile probe;
//! derivation notes in `.scratch/parquet-deformations/`):
//!
//! ```text
//! | substrate          | closed δ          |
//! |--------------------+-------------------|
//! | square (4.4.4.4)   | 45.0, 90.0        |
//! | truncated-square   | 45.0, 90.0        |
//! | trihexagonal       | 60.0, 90.0        |
//! | truncated-hexagonal| 60.0              |
//! ```
//!
//! **Discrete, not an interval.** The brief's envelope framing — "which
//! [δ_min, δ_max] keeps every tile's strand structure closed" — is
//! geometrically impossible on these substrates: for a tile to close, its
//! rays must exit at the midpoints of the adjacent edges exactly (the two
//! rays at a shared midpoint are point-reflected in each other), which
//! happens only at a discrete set of angles (45°/90° on squares, 60°/90° on
//! 3.6.3.6, 60° on 3.12.12). Every other angle exits at non-midpoint
//! boundary points → degree-1 exits / end-caps → open strands. There is no
//! contiguous interval of closed angles, so the envelope is a **discrete
//! point set**, not [δ_min, δ_max]. A 0.1° fine scan around each closed
//! point confirms the debris reappears immediately (e.g. at 44.9° and
//! 45.1° on the square substrate). 90° is the degenerate median-chord
//! regime (the two lean rays collapse onto the inward normal and meet at
//! the tile centre) — closed, but degenerate.
//!
//! **Policy (Phase B):** profiles are always clamped to this envelope —
//! values snap to the nearest closed angle (see
//! [`tess_core::strapwork::snap_to_envelope`]). Clamping to the envelope is
//! the correct behaviour, not a limitation: it is what keeps every wall
//! set closed on the deformed strip.
//!
//! Substrates without a validated envelope (any registered tiling not in
//! the table) return `None` — an honest absence, never a guess.

/// The validated closed contact angles (degrees) for a registered
/// strapwork substrate — the Phase-A envelope. Sorted ascending; empty
/// never returned. Returns `None` for substrates without a validated
/// envelope.
pub fn envelope_for(substrate_id: &str) -> Option<&'static [f64]> {
    match substrate_id {
        "square" => Some(&[45.0, 90.0]),
        "truncated-square" => Some(&[45.0, 90.0]),
        "trihexagonal" => Some(&[60.0, 90.0]),
        "truncated-hexagonal" => Some(&[60.0]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::envelope_for;

    #[test]
    fn canonical_angles_match_layer_mode_anchors() {
        // The layer-mode engine anchors (M57) close at 45° / 60° / 60°.
        assert_eq!(envelope_for("truncated-square"), Some(&[45.0, 90.0][..]));
        assert_eq!(envelope_for("trihexagonal"), Some(&[60.0, 90.0][..]));
        assert_eq!(envelope_for("truncated-hexagonal"), Some(&[60.0][..]));
        // Plain square grid (the Fig. 3.7 canonical smoke-test substrate).
        assert_eq!(envelope_for("square"), Some(&[45.0, 90.0][..]));
    }

    #[test]
    fn envelope_angles_are_valid_and_sorted() {
        for id in [
            "square",
            "truncated-square",
            "trihexagonal",
            "truncated-hexagonal",
        ] {
            let e = envelope_for(id).expect(id);
            assert!(!e.is_empty(), "{id}");
            assert!(e.windows(2).all(|w| w[0] < w[1]), "{id} sorted: {e:?}");
            for &a in e {
                assert!((0.0..=90.0).contains(&a), "{id}: angle out of range: {a}");
            }
        }
    }

    #[test]
    fn unknown_substrates_have_no_envelope() {
        assert_eq!(envelope_for("unknown-substrate"), None);
        assert_eq!(envelope_for("pic-4-8-8"), None); // tiling-mode id, not an engine substrate
    }
}
