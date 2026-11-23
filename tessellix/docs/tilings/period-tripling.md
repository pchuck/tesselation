# Period-tripling (2D) substitution

`period-tripling` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

marginal supersubstitution: one unit-square prototile carrying 2 label classes (A gray, B blue — the Bielefeld header's "2 prototiles"), λ = 3 substitution into 9 children — parent A → 3×3 with B on the main diagonal (6 A + 3 B), parent B → 3×3 all A; S = [[6,9],[3,0]] (child × parent), column sums 9 = λ², PF eigenvalue 9 = λ², char poly x²−6x−27 = (x−9)(x+3) — second eigenvalue −3 = −λ, the marginal case, pinned by exact 2-permutation determinant; the rule was RECONSTRUCTED from the Bielefeld rule image (the entry's Info section is empty — no tags, no references, no source citable) and pinned by an 81-offset window match of the entry's patch image against the simulated depth-4 tiling (exactly the predicted period lattice); limit-periodic via approximating periodic tilings — the depth-g patch's period lattice is exactly ⟨(1,−1), (3^g, 0)⟩ (measured g = 1..5, size 3^g, no other periods): period-1 along (1,−1) at all depths, axis-aligned (3^g, 3^g) strictly tripling per level (vs period-doubling's ratio-1 and domino's ratio-2); 1D shadow σ: A → ABA, B → AAA (the 3-adic cousin of period-doubling's A → AB, B → AA), 1-periodic along (1,−1), 3-adic limit-periodic across it; the marginal second eigenvalue leaves exactly one ambiguous line — x+y = 0 carries B at odd depth / A at even depth, two limit tilings up to translation; d = 5 single depth band (9^5 = 59049 ≤ MAX_TILES), closed form B ⟺ v₃(x+y mod 3^d) odd, exact Z² arithmetic end to end (M146)

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling period-tripling --mode proto --out period-tripling.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
