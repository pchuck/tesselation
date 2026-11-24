# Girih Decagon Patch (Darb-i Imam, two-level)

`girih-decagon-patch` · family [Islamic](../families/islamic.md) (`islamic`)

finite two-level girih patch — the documentary decagon-and-bowtie
figure of Lu & Steinhardt, Science 315, 1106 (2007): **11 large tiles**
(the seed decagon at the origin plus its 10 edge-adjacent bow-ties,
rotations 1, 3, …, 19 — the certified edge-neighbourhood of the seed in
the generation-1 lineage of `girih-quasiperiodic`) with every large
tile replaced by its **closed cluster** of small girih tiles — the real
Darb-i Imam children plus the straddling edge trio on every unique
large-tile edge. The union is **611 tiles: 270 bobbins (hexagons), 220
bow-ties, 121 decagons**, exact over K = Q(2cos 18°) (global
denominator 64, floats only at emission), a duplicate-free topological
disk (χ = 2, V = 1730, E = 2340, boundary 530 edges), edge-to-edge and
T-junction-free, exactly 10-fold symmetric about the seed decagon, and
it covers every large-tile outline exactly once

## Region and assembly

The minimal region containing one full large decagon plus its adjacent
large bow-ties: 1 central DEC + 10 BOW (empirically **all** edge-
neighbours of the seed decagon in the certified girih_qc generation-1
lineage are bow-ties — a perfect 10-fold ring, re-verified exactly
here: each bow-tie shares exactly one unit edge with the decagon and
every decagon edge has exactly one bow-tie).

Each large tile expands by **one substitution step** of the certified
`girih_qc` tables: real children at `IF·pos + rot_kp(rot, off)`
(`REAL_DEC` 181 + `REAL_BOW` 25 rows, copied verbatim), plus the
edge-marker trio `EDGE_RULE` (HEX–DEC–HEX) on every **unique** large-
tile edge — the children-only union would leave the notch gaps where
child clusters protrude across parent boundaries, while the closed
cluster (children + boundary trios, certified per parent type by
`substitution_tables_close_exactly`) covers each large tile exactly
(431 children + 60 unique edges × 3 trio tiles = 611, zero duplicates,
verified exactly by the `(type, rotation, exact position)` key).
467 of the 611 tiles are generation-2 tiles of the single-lineage
tiling; the other 144 are the boundary trios on the ring's outer edges,
genuine full-plane tiles beyond the seed window.

## Algorithm

Data-table `Tiling` (not a `LatticeTiling`): the 611 tile poses are
assembled once (lazily) from the copied exact tables and cached;
`generate` transforms them by the similarity map (scale + rotation
params, `SIMILARITY_PARAMS`) and emits exactly those whose aabb meets
the query box. The domain IS the patch — views beyond the rim return no
tiles (the `jeandel-rao` precedent); a view around the rim renders all
611 tiles. Colouring: DEC → 0, HEX → 1, BOW → 2 (`proto` mode default).

## Verification

The Python pre-Rust battery (stdlib, exact K-tuples) and its in-module
Rust mirror certify, at the numbers quoted above: duplicate-free closed
clusters with census (270, 220, 121); every tile edge exactly unit
(pdot == 1 in K); directed-edge census 4150 with 1810 matched interior
pairs and 530 boundary edges; **0 T-junctions** (exact collinearity +
position); all 1730 vertex fans legal (single-cycle / single-chain
corner maps, 1200 interior vertices closing 360°); χ = 2; 10-fold
symmetry exact as a pose set (rot classes modded by prototile
symmetry); per-tile exact-K shoelace areas equal the table areas to
< 1e-12 (total 1794.340677298660) and the 530-edge boundary cycle's
area equal to the tile sum to 1.1e-12; the large DEC outline area
equals IF²·A_DEC = 552.267902440007 (< 1e-9) and all ten large BOW
outlines IF²·A_BOW; coverage-sampling over the large region (33 976
samples, irrational-offset grid) **exactly once** per point, and a
61 625-sample sweep of the whole bbox with **0 overlaps** and **0
region gaps**. Standard trio: `check_covers` / `check_rotated` /
`check_deterministic` on windows inside the seed decagon's inflated
inradius (13.04).

**Strapwork bands (stretch goal) not shipped:** the girih strap chords
of the small tiles cross inside the decagon's {10/3} star polygon;
turning that weave into non-overlapping strap quads requires
planarisation of the crossings, which breaks the exact-partition
contract (straps + infill must tile the region with no gaps or
overlaps) at the pinned offset — reported rather than shipped as an
overlapping overlay, per the brief. The tiles-only patch is
contract-legal by the closed-cluster cover law.

## Sources

- LS07: J. Lu, P. J. Steinhardt, "Decagonal and Quasi-crystalline
  Tilings in Islamic Art", Science 315, 1106–1109 (2007) — the
  two-level girih figures (documentary reference for the LOOK; the
  geometry authority is `girih_qc.rs`'s certified tables).
- Darb-i Imam substitution: as implemented in `girih_qc.rs` (M18).

## See also

- The infinite relative: [`girih-quasiperiodic`](girih-quasiperiodic.md) — this patch's tables are its generation-1/2 lineage
- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling girih-decagon-patch --mode proto --out girih-decagon-patch.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
