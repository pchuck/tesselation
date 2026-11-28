# Star and Cross ({8/2})

`star-cross-8-2` · family [Islamic](../families/islamic.md) (`islamic`) · wallpaper group `p4m` · square lattice

Classical Islamic star and cross built on the {8/2} octagram — two-square compound stars (90° points, notch/tip √(2−√2)) kissing tip-to-tip, with 90°-pointed concave crosses in every gap; rigid exact geometry, one star + one cross per square-lattice cell (p4m). Vertex figures 90.90.90.90 / 90.270 / 135.225.

This is the **other** star-and-cross: [`star-cross`](star-cross.md) is the
`{8/3}` octagram reading (notches at the crossings of the density-3
chords, notch/tip √((2−√2)/2) = 0.541196…, 45° tips, a measured
one-parameter family). This card is the compound `{8/2}` — the union
outline of two equal squares about a common center, the second rotated
45° — whose notches are the crossings of the **skip-2** tip chords
(the squares' own sides): notch/tip √(2−√2) = cos(π/4)/cos(π/8) =
0.765367… and 90° points. The A4 review asked for both readings; the
discrimination between them is pinned in both directions
(`star_cross_star_is_the_8_3_octagram_not_the_8_2_compound` in the
`{8/3}` card, `star_cross_8_2_star_is_the_two_square_compound` and
`star_cross_8_2_and_8_3_are_different_figures` here).

## Construction

Lattice constant 1, square lattice; the fundamental cell emits **one
star + one cross** (2 tiles).

- **Star** — the `{8/2}` compound at the kissing radius 1/2: tips at
  radius 1/2 on the 45° grid (so stars kiss tip-to-tip at every
  cell-edge midpoint), notches at radius √(2−√2)/2 = 0.382683… between
  them. With q = √2/4, the CCW outline centered at the lattice point
  starts tip (1/2, 0), notch (q, 1/2 − q), tip (q, q), notch
  (1/2 − q, q), … (D8 symmetric, 16-gon).
- **Cross** — the interstice is a concave 16-gon contained in one
  cell: four 90°-pointed arms reaching the cell-edge midpoints
  (1/2, 0), (1, 1/2), (1/2, 1), (0, 1/2); reflex 270° corners at the
  four diagonal star tips (q, q), (1−q, q), (1−q, 1−q), (q, 1−q);
  convex 135° corners at the eight star notches.

Every one of the 32 prototile edges has the same length (√2−1)/2 =
0.207107…, so star and cross meet edge-to-edge and each star edge is
glued to exactly one cross edge.

## Exact geometry

All vertices lie in Q(√2) with denominator 4 — the construction is
exact, not measured. Areas: star 2 − √2 = 0.5857864376269049, cross
√2 − 1 = 0.4142135623730951, sum **exactly 1** = |det(a₁, a₂)|.

The kissing radius is *forced*: the two squares' sides are the skip-2
tip chords of the outline, so their crossings (the notches) are
determined by the tip radius, and tip-to-tip contact between adjacent
stars fixes that radius at 1/2. The tiling is therefore rigid — it
carries no shape parameter, only the standard scale/rotation
similarity controls. (Contrast the `{8/3}` sibling: there the notch is
a chord crossing of two *independent* squares only at T = 1/2, and the
family sweeps T ∈ (0, 1/2] with measured decimal data standing under
the M35 measurement exception.)

## Verification

Python stdlib battery over exact Q(√2) arithmetic (shoelace, squared
edge lengths and collinearity computed symbolically in the field;
decimals only for angle classification), 8×8-cell patch (exceeds the
±3×3 requirement), 128 tiles:

- area: exact shoelace equals 2−√2 and √2−1; per-cell sum − |det| = 0
  (exactly, not ~1e-15); patch mean error 0;
- edges: all 32 prototile edges squared equal (3−2√2)/4; interior-edge
  histogram {valence 2: 900, boundary 1: 248}; all 85 core tiles have
  every edge valence 2;
- vertices: 264 interior vertices, angle sum 360° at every one;
  vertex-figure multiset 90.90.90.90 (×40, the kissing tips),
  90.270 (×64, diagonal star tip + cross reflex corner), 135.225
  (×160, cross convex corner + star notch reflex);
- coverage: 24 336 shifted-irrational-offset grid samples, 0 points
  covered ≠ 1 time;
- structure: notches lie exactly on the skip-2 chords (squares' sides)
  and a macroscopic 0.07926 from the nearest skip-3 chord.

Rust pins (module `tests`): covers-region / rotated-scaled /
deterministic standard checks; radical coordinate, radius, ratio
(0.7653668647301796), apex-angle, edge-length and area pins for star
and cross; the `{8/2}`-vs-`{8/3}` structural discriminator (skip-2-on
/ skip-3-off + 90° apex) and a side-by-side test against the emitted
`{8/3}` star.

## Sources

- The two-squares rotated 45° construction of the eight-pointed star
  and the pointed cross filling every gap is the classical
  star-and-cross tessellation of Islamic (Ilkhanid-era) tilework;
  see the craft-geometry literature on the eight-point star
  ("two overlapping squares") and the `star-cross` sibling's sources.
- `{n/k}` star-polygon notation and the `{8/2}` reading as the
  compound of two squares: P. R. Cromwell, *Patterns and Tilings of
  the Plane* (Cambridge, 1997).
- The registry-side `{8/3}`-vs-`{8/2}` discrimination follows the
  review-A4 record in `crates/tess-tilings/src/families/islamic/`
  (`zellige.rs`, "Which polygram (review A4)").

## See also

- The `{8/3}` sibling card: [star-cross](star-cross.md) — measured
  one-parameter family, 45° tips; do not confuse the two figures.
- [`khatam-girih`](khatam-girih.md) — girih kitab of the kissing
  `{8/3}` case; [`pic-4-8-8`](pic-4-8-8.md) — the rosette composition
  whose octagram core is the `{8/2}` figure (per the `zellige` module
  note).
- Family doc (catalog table + build arcs): [docs/families/islamic.md](../families/islamic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling star-cross-8-2 --mode proto --out star-cross-8-2.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
