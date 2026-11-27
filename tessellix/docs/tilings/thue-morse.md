# Thue-Morse (2D product) substitution

`thue-morse` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

non-periodic product substitution: the classic 1D Thue–Morse morphism 0 → 01, 1 → 10 (Bielefeld entry "Thue Morse" — 1D-only; the 2D rule pinned against JS99, Allouche & Shallit) squared as τ⊗τ on Z² unit squares — 4 prototile label classes (00/01/10/11, class = 2·(x⊕c) + (y⊕r), seeds 0–3) over one square prototile, λ = 2 into a 2×2 frame; S = the all-ones 4×4 (Steinhaus property — every parent block contains each child class exactly once), PF eigenvalue 4, char poly x⁴−4x³ (eigenvalues {4, 0, 0, 0}, rank 1, pinned by exact 24-permutation determinants at x = 0..6: [0, −3, −16, −27, 0, 125, 432]); non-periodic at every depth — NO proper translation period at any depth g = 1..6 (all (g+1)²−1 proper candidate shifts rejected per depth, 133 cumulative, zero survivors; singular continuous diffraction, not pure point) — unlike the limit-periodic period-doubling cousin; exact integer arithmetic on Z² (M143)

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling thue-morse --mode proto --out thue-morse.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
