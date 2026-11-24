# Islamic-geometric family

Islamic-geometric constructions: star-and-cross, girih strapwork, the PIC/Hankin inference engines and their substrate tables, khatam, najm — plus the quasiperiodic girih member.

Code: `crates/tess-tilings/src/families/islamic/` — cargo feature `islamic` (default-on).
Adding a tiling here touches only this family: `families/islamic.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (30 tilings)

### Periodic decorative families

| id | motif | group | notes |
|---|---|---|---|
| `star-cross` | {8/3} star and cross | p4m | one-parameter star radius: gapped → kissing (khatam) |
| `hexagram` | hexagram and hexagon ({6/2}) | p6m | one-parameter star tip radius: gapped → kissing hexagram-and-hexagon tiling; stars on the hexagonal lattice, interstice split into two hexagons at the kissing radius |
| `girih-shesh-band` | girih strapwork band | p2 | single shesh-band hexagon, centered rectangular lattice (M76: kind corrected from oblique) |
| `girih-star-cross` | 10-pointed girih star-and-cross strapwork (periodic) | cmm | measured classical panel; 545-tile cell data table on the (77,45)/(0,90) centered-rectangular basis (M35; M76: group pmm → cmm, lattice kind → centered rectangular) |
| `girih-w85` | 12-pointed girih star interlace | p6 | Wichmann data18/W85 (Alhambra Mexuar): exact Q(√3) strap-line grid (L = 283.92, strap 9, all vertices (p+q√3)/50), 42 tiles per hexagonal cell; motif is *632 but the drawn over/under interlace is chiral → p6 (M52) |
| `pic-4-8-8` | Hankin PIC fourfold khatam (median) | p4m | polygons-in-contact strapwork on the 4.8.8 substrate at δ = 45°: rays run edge-midpoint to edge-midpoint ({8/2} two-square octagram per octagon, midpoint chords per square), face-to-face partition into octagon + small square + 4 house pentagons + 8 star tips, 14 faces per A·Z² cell (A = 1+√2), exact Q(√2) with all vertices (p+q√2)/4; un-interlaced, mirrors and glide probe verified (M53) |
| `pic-3-6-3-6` | Hankin PIC sixfold khatam | p6m | polygons-in-contact strapwork on the trihexagonal 3.6.3.6 substrate at δ = 60°: rays run edge-midpoint to edge-midpoint ({6/2} hexagram per hexagon, midsegments per triangle), face-to-face partition — the wall network *is* a half-scale 3.6.3.6 — into 4 hexagram-core hexagons + 6 hexagram tips + 2 midsegment triangles, 12 faces per ⟨(2,0),(1,√3)⟩ cell, Σ = 2√3 = |det| exact over Q(√3) with all vertices on the (p+q√3)/4 grid; un-interlaced, rot60/rot120/rot180 + five mirrors probe verified (M54) |
| `pic-3-12-12` | Hankin PIC twelvefold khatam | p6m | polygons-in-contact strapwork on the truncated-hexagonal 3.12.12 substrate at δ = 60°: the tangent-chord theorem closes a midpoint ray onto another midpoint only at δ = 180°·k/n, so δ = 60° is the single angle that closes both polygons — 12 chords `M_i→M_{i+4}` per dodecagon (the {12/4} rosette, four overlaid triangles) plus the triangle midsegments; face-to-face partition into 45 faces per ⟨(3/2+√3, 1+√3/2),(0,2+√3)⟩ cell in six orbits (1 rosette-core dodecagon + 6 vertex pentagons + 12 band quads + 2 medial triangles + 12 inner quads + 12 star notches), Σ = 6 + 7√3/2 = |det| exact over Q(√3) with all vertices on the (p+q√3)/24 grid and corners only 30–150°; un-interlaced, rot60/rot120/rot180 + three mirrors probe verified and 7 negative probes pin the group to p6m (M56) |
| `hankin-4-8-8-rosette` | Hankin inference fourfold rosette | p4m | M94 inference on 4.8.8 (θK = 67.5°, δ = 0): 8-petal rosette + core octagon per octagon, 4-pointed star + core diamond per diamond; exact Q(√2) (§44 config A) |
| `hankin-4-6-12` | Hankin inference twelvefold | p6m | M94 inference on 4.6.12 (θK = 60°, δ = 0): 12-petal dodecagon rosette, 4-pointed stars in the squares, 6-petal hexagon rosettes; exact Q(√3) (§44 config B) |
| `hankin-hex-two-point` | Hankin two-point weave | p6m | M94 two-point inference on the unit hexagon (θK = 45°, δ = 0.5): 12 bracket-and-diamond faces around a central dodecagon; Kaplan Fig 3.6 (§44 config C) |
| `hankin-3-12-12` | Hankin twelvefold khatam (3.12.12) | p6m | M99 inference on 3.12.12 (θK = 45°, δ = 0): 24-gon star core + 12 petals per dodecagon, 4-pointed stars + hexagon tips per triangle orbit; exact Q(√3) (§53) |
| `hankin-3-6-3-6` | Hankin sixfold hexagram (3.6.3.6) | p6m | M100 inference on the trihexagonal 3.6.3.6 substrate (θK = 60°, δ = 0): each hexagon → regular hexagram — 12-gon core with 60° tips and 240° notches (vertices alternating on circles of radius 1/2 and √3/2) + six 60°-lozenge rhombi (7 faces); the two equilateral triangle tiles pair with no contact rays and stay single faces — 9 faces per cell ⟨(2,0),(1,√3)⟩ (det 2√3), exact Q(√3); θK = 60° chosen over 30°/45° (no star: core tips 120°/90°) and 67.5°/90° (sharper variant / star collapse) for the classical {6/2} hexagram; p6m verified on a ±5-lattice window (§57) |
| `hankin-3-4-6-4` | Hankin sixfold rhombitrihex motif (3.4.6.4) | p6m | M100 inference on the rhombitrihexagonal 3.4.6.4 substrate (θK = 60°, δ = 0): each hexagon → the regular hexagram (12-gon core + six 60°-lozenge rhombi), the three unit squares pair with no contact rays and stay single faces, each equilateral triangle in the side-1/2 4-subdivision limit (star tips reach the edge midpoints exactly — 4 faces) — 18 faces per cell ⟨(1.5+√3/2, √3/2+1/2),(0,1+√3)⟩ (det 4√3/2+3), exact Q(√3); θK = 60° chosen over 30° (30° slivers), 45° (no hexagon star), 67.5° (45° tips), and 90° (star collapse); p6m verified on a ±5-lattice window (§56) |
| `hankin-square-two-point` | Hankin two-point square weave (4.4.4.4) | cmm | M99 two-point inference on the unit square (θK = 45°, δ = 0.5): decagonal star per tile — 4 corner triangles + 2 pentagons + 10-gon core (§54) |
| `hankin-4-8-8-rosette-transform` | Hankin rosette-transform khatam | p4m | Kaplan rosette transform (diss. §3.8) on the 4.8.8 substrate at θK = 45°, δ = 0: each octagon → concentric inner octagon (r′ = r·cos(π/8)/(1+sin(π/8)), rotated π/8, spokes = half the inner side) + 8 pentagons; each square → 4 half-squares of side 1/2 (all four bisectors pass through the centre — Kaplan's cluster heuristic fires: 4 of the 6 bisector-pair intersections, one cluster) — 93-tile transformed table per ⟨(1+√2,0),(0,1+√2)⟩ cell (16 triangles, 36 quads, 32 pentagons, 8 hexagons, one 16-gon), edge-to-edge (0 unpaired, 0 T-junctions, w = 4), area census |det| = 3+2√2, exact Q(√2); then M94 inference at θE = 45°; transformed table p4m-exact to 6.5e-16 (M99) |
| `hankin-3-3-3-4-4` | Hankin elongated-triangular motif (3.3.3.4.4) | cmm | M107 inference on the elongated-triangular 3.3.3.4.4 substrate (θK = 60°, δ = 0; all-CCW in-cell table — the CW down-triangle and CW square emit outward rays and pair with nothing at every probed angle): both triangle tiles in the side-1/2 4-subdivision limit (4 equilateral faces each), the square as four corner quadrilaterals + central octagon — 13 faces per cell ⟨(1,0),(1/2,1+√3/2)⟩ (det 1+√3/2), exact Q(√3); cmm verified exactly on a 551-face window (§63) |
| `hankin-3-3-4-3-4` | Hankin snub-square motif (3.3.4.3.4) | p4g | M111 inference on the snub-square 3.3.4.3.4 substrate (θK = 60°, δ = 0; all twelve instances stored CCW in their in-cell positions): each square in its five-face motif (four corner quadrilaterals + central octagon), each triangle in the side-1/2 4-subdivision limit (4 equilateral faces each) — 52 faces per cell ⟨(1+√3,0),(0,1+√3)⟩ (det 4+2√3); tiles and lattice in Q(√3), all face vertices in (ℤ+ℤ√3)/4 (√2 and √6 coefficients identically zero in the full Q(√2,√3) audit; √2 only as the 45° glide-ray direction); p4g verified exactly on a 6292-face window: face-set translations the centered lattice Λ′ = ⟨(L/2,L/2),(L/2,−L/2)⟩ with L = 1+√3, 4-fold centers {(L/4,L/4+½),(L/4,3L/4+½),(3L/4,L/4+½),(3L/4,3L/4+½)}, 2-folds = those ∪ {(0,½),(0,(L+1)/2),(L/2,½),(L/2,(L+1)/2)}, glides on 45° lines y−x ≡ L/4+½, 3L/4+½ and 135° lines y+x ≡ L/4+½, 3L/4+½ with ¼-period shifts ±(L/4,∓L/4) — off the L/8 grid; no mirrors (§66, provisional) |
| `khatam-girih` | khatam ({8/3}) girih-tile decomposition | p4m | classical kissing star-and-cross cut into 8 rhombi + square + 4 pentagons per cell; exact Q(√2) kitab (M36) |
| `najm-63-2e-0-3e-ac` | Najm template ([6,3]; 2e, 0, 3e; AC) | p6m | najm-generator substrate (diss. §3.6.1 Fig. 3.21): dodecagons + enneagons sharing equal chords, x = 2/(cot 15° + cot 20°), concave equilateral bowties (70°/220°) at the 2-fold points; 6 tiles/cell, |det| = 3√3/2 (M99) |
| `najm-44-3e-0-0-a` | Najm template ([4,4]; 3e, 0, 0; A) | p4m | najm-generator substrate (diss. §3.6.1 Fig. 3.22): dodecagons r = √6−√2 on 2Z² sharing chords across odd grid lines + concave equilateral 4-pointed stars (side 4−2√3, 60° tips, 210° notches); exact Q(√2,√3) (M99) |
| `girih-bowtie` | girih bowtie (papillon) prototile tiling | cmm | the concave equilateral girih hexagon (216°,72°,72°,216°,72°,72°, unit edges) tiles the plane by translation alone on the centered rectangular lattice ⟨(cos36°+cos72°, sin36°−sin72°), (1+cos36°, sin36°)⟩; det = tile area exact over Q(√5); vertex figure 72.72.216 at every interior vertex; tile mirrors along both rectangle axes give cmm |
| `girih-decagon` | girih decagon prototile tiling | cmm | the regular girih decagon (144°, unit edges) cannot tile alone; its primitive periodic edge-to-edge arrangement is the two-tile cell of one decagon plus one forced bowtie infill on the centered rectangular lattice ⟨(0, −2(sin72°+sin36°)), (1/2+3cos36°, sin72°+2sin36°)⟩; det = decagon+bowtie area exact over Q(√5); vertex figure 10.6.10 (two decagons + one bowtie close 144+72+144); two perpendicular mirror families with interleaved glides give cmm |
| `girih-pentagon` | girih pentagon prototile tiling | cm | the regular girih pentagon (108°, unit edges) cannot tile alone; its primitive periodic edge-to-edge arrangement is the seven-tile cell of a pentagon pair + two shesh-band hexagons + three girih rhombi on the centered rectangular lattice ⟨(−cos36°, −2sin72°−sin36°), (2+√5, 0)⟩; det = the seven tile areas exact over Q(√5); vertex figure 5.5.4.4 (two pentagons + two rhombi close 108+108+72+72); one mirror family with interleaved glides and no rotations gives cm |
| `girih-rhombus` | girih rhombus prototile tiling | cmm | the thin equilateral girih rhombus (72°,108°,72°,108°, unit edges) tiles the plane by translation alone on the centered rectangular lattice ⟨(1,0), (cos72°, sin72°)⟩ spanned by two adjacent edge directions; det = sin 72° = tile area exact over Q(√5); vertex figure 72.72.108.108 at every interior vertex; the diagonally orthogonal basis (diagonals U0±U2) with tile D2-mirrors along both rectangle axes gives cmm |

### Quasiperiodic member: girih star-and-cross {10/3} (category Quasiperiodic, family islamic)

| id | motif | notes |
|---|---|---|
| `girih-quasiperiodic` | girih star-and-cross {10/3} (quasiperiodic) | quasiperiodic; Darb-i Imam 3-prototile substitution (Lu & Steinhardt 2007, IF = 2φ³); exact 10-fold seed rosette, edge-to-edge at every generation |
| `andritz-bowtie-hexagon` | bow-tie + elongated-hexagon girih (quasiperiodic) | quasiperiodic; Andritz 2016 fivefold substitution (Tilings Encyclopedia, Feb 2016), IF = φ², matrix [[2,3],[3,5]] with PF = φ⁴; exact Z[ζ₅] σ-frame walk from a 27-tile seed core, walk reproduces the decoded Bielefeld reference patch pose-for-pose, edge-to-edge at every level |
| `andritz-bowtie-hexagon-10gon-1` | bow-tie + elongated-hexagon + regular-decagon girih (quasiperiodic) | quasiperiodic; Andritz 2016 fivefold substitution 10gon-1 (Tilings Encyclopedia, Feb 2016), IF = φ², matrix [[2,3,0],[2,2,1],[5,0,6]] with PF = φ⁴; exact Z[ζ₂₀] σ-frame walk (rotating Ψ(r) frames, absolute decagon flower) from a 14-tile seed core, walk reproduces the decoded Bielefeld reference patch pose-for-pose, edge-to-edge at every level |
| `andritz-bowtie-hexagon-10gon-2` | bow-tie + elongated-hexagon + regular-decagon girih (quasiperiodic) | quasiperiodic; Andritz 2016 fivefold substitution 10gon-2 (Tilings Encyclopedia, Feb 2016), σ-frame map z ↦ τ − λz with λ = 3 + 2φ, cluster census [[11,14,2],[12,22,4],[35,35,26]] with PF = 43.151 (a shared covering: clusters share boundary tiles, so generation is by the verified walk, not a disjoint inflation); exact Z[ζ₂₀] σ-frame walk with a fitted per-slot orientation law from a 13-tile seed core, walk reproduces the decoded Bielefeld reference patch pose-for-pose, edge-to-edge at every level |
| `andritz-bowtie-hexagon-10gon-3` | bow-tie + elongated-hexagon + regular-decagon girih (quasiperiodic) | quasiperiodic; Andritz 2016 fivefold substitution 10gon-3 (Tilings Encyclopedia, Feb 2016), σ-frame map z ↦ μz + τ with μ = φ⁴·e^{72°i}, cluster census [[19,19,7],[26,29,9],[60,45,31]] mined from the decoded 957-tile patch by unanimous union votes, region-killed to slots landing inside the inflated parent (|w|/2 ≤ φ⁴·circ) and configuration-killed against far-field overlaps (a shared covering: 42.7 % of tiles are claimed by several parents, so generation is by the verified walk, not a disjoint inflation); exact Z[ζ₂₀] σ-frame walk with frames Ψ_HEX = (8 − 8r) and Ψ_BOW = (4 − 8r) mod 20 from a 21-tile seed core, walk reproduces all 957 reconstructed tiles pose-for-pose by level 2 with zero false claims, edge-to-edge at every level |

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

### 9. Girih star-and-cross redo ✅ (M18 redo)

- **Motif:** the girih star-and-cross — the quasiperiodic girih pattern
  of Lu & Steinhardt (*Science* **315**, 1106 (2007)) as realized in the
  Darb-i Imam shrine, Isfahan: the classical 10-fold rosette (a decagon
  ringed by bow-tie "petals") at the seed, quasiperiodic order
  throughout. The original M18 entry embedded a static 41-tile patch
  whose individual tiles were valid but whose arrangement was wrong
  (no 10-fold symmetry, no growth); this redo replaces it with the
  genuine **Darb-i Imam 3-prototile substitution**, so the entry now
  generates the actual quasiperiodic tiling.
- **The substitution system (derived from Greg Egan's Darb-i Imam
  applet source, then re-derived exactly):** three unit-edge girih
  prototiles — DEC (regular decagon, circumradius φ), HEX (bobbin
  hexagon), BOW (bow-tie hexagon); 20 orientations in 18° steps;
  inflation factor IF = 2·(2+√5) = 2φ³ ≈ 8.4721. Each parent expands to
  a fixed cluster of unit children placed in the parent's frame scaled
  by IF; clusters protrude across the parent boundary and adjacent
  protrusions are *identical tiles* (this is what meshes the clusters).
  Seams are filled by the **edge rule**: every tile also emits one
  *edge marker* per interior edge of its closed cluster, and each
  marker substitutes to the straddling trio `[HEX, DEC, HEX]` at
  offsets ∓(3φ+1)/2 along the edge. The bookkeeping is exact: the
  markers of a generation are precisely the undirected edge set of
  that generation's patch (verified bijectively at generation 2:
  55 080 markers = 55 080 patch edges, zero duplicates).
- **Exact arithmetic:** positions live in K = Q(a), a = 2cos(π/10)
  (degree 4), stored as integer 4-tuples over (1, a, a², a³) at a
  global denominator 64 — every construction quantity has true
  denominator dividing 8, so the fixed-point representation is closed
  and exact (all generated coefficients stay ≡ 0 mod 8, inductively).
  The child tables were derived by BFS-propagating centres through
  exact edge-contact equations and cross-checked against Egan's
  doubles to ≤ 1.1e-14; rotations multiply exactly in K
  (a⁴ = 5a² − 5). Table counts: HEX parent → 57 children + 210
  markers, BOW → 25 + 114, DEC → 181 + 660.
- **Verification (Python prototype, then re-run in Rust):**
  generation 1 = 211 tiles (90 HEX, 80 BOW, 41 DEC) + 660 markers;
  generation 2 = 16 531 tiles (7 350, 6 740, 2 441) + 55 080 markers;
  for both: every edge unit (exact dot product), zero duplicate tiles,
  every interior edge matched by opposite orientations, zero
  T-junctions, legal vertex fans (disjoint sectors; interior vertices
  one closed 20-class cycle), Euler characteristic 2, exact 10-fold
  rotational symmetry (canonical per type: DEC 36°, HEX/BOW 180°).
  Frozen core: P₂ = P₃ exactly up to radius 79 (P₃ = P₄ to ≈ 643) —
  inside the core the tiles are depth-independent. Parent clusters
  cover their parents (dense sampling, all 3 types), which makes the
  covered-disk growth structural: measured ι₂ = 111.30, ι₃ = 910.01
  (ratio 8.18×/generation).
- **Generation for a view:** one global lineage from a seed unit
  decagon at the origin (plus its 10 boundary edges as markers).
  Depth selection from a conservative covered-radius table
  (111·8^(k−2), k ≤ 12 ⇒ covered radius ≈ 1.2e11), then the patch is
  built top-down with an **S-box window cascade** (each level keeps
  only objects whose children can reach the view; child offsets are
  bounded by 13.31 in child units, so an inter-level margin of
  13.31/IF ≈ 1.571 suffices — the cascade was proven equal to the
  full lineage exactly at generations 2 and 3, on- and off-origin).
  Acceptance is exact clipped-area equality against the view box
  (valid to ~1e5 in tiling coordinates; beyond that the depth-12
  patch is verified structurally — at rcorr ≈ 7.2e10 the position
  coefficients reach ~1e13 and kmul intermediates stay below
  ~3e16, two orders inside i64). Emission is unit-edge at every
  depth; zooming deepens the patch; the frozen core is
  depth-independent, so nested views are bit-identical there.
- **Implementation:** `girih_qc.rs` (feature `girih`), 3 prototiles
  (proto 0 = DEC, 1 = HEX, 2 = BOW; seed = proto). 12 Rust tests: the
  exact K arithmetic (unit edges, exact rotation tables), the
  substitution-table closure (markers == interior edges, bijective,
  offset bounds), generations 1–2 full structural battery, cascade ==
  full lineage, core depth-stability, view coverage at depths 2–5,
  depth-12 structural validity, view guards, and the standard trio
  plus distant-view coverage. TUI export visually verified against the
  Python prototype's render (10-fold rosette, tile-for-tile).
- **Status:** committed (M18 redo). 33 tilings, 197 tests.


> Follow-up: measured ornamental panel — ✅ complete (M35)
>


### 10. Girih star-and-cross, 10-fold periodic (measured) ✅ (M35)

- **Motif:** the classical *periodic* 10-pointed girih star-and-cross
  strapwork: 10-pointed stars on an oblique lattice, joined by chord
  straps with bobbin (elongated hexagon) motifs between the stars and
  small square/diamond ground faces at the diagonal positions — the
  well-known strapwork wallpaper that the quasiperiodic Darb-i Imam
  entry (`girih-quasiperiodic`, M18) is the aperiodic cousin of. Unlike
  M18's exact substitution system, this entry reproduces a measured
  classical panel as a data table: the artwork is decoded from a
  reference image and re-emitted as a `LatticeTiling` cell.
- **Decoded geometry:** reference panel registered at 1 px/unit;
  fundamental cell `[-38.5, 38.5] × [-45, 45]` (area 6930) on the
  oblique lattice `a1 = (77, 45)`, `a2 = (0, 90)`; star centres at the
  index lattice, bobbin centres at the half-lattice holes; wallpaper
  group **pmm** (mirror axes through the stars, confirmed numerically:
  mirror residuals 0.008–0.02 vs 0.005 noise baseline). Cell contents:
  99 white ground faces (the central 10-pointed pinwheel ≈ 443 u²,
  4 diagonal faces ≈ 320 u², 4 petals, 2 side corridors, 4 corners,
  small squares/diamonds) and 446 ink strap pieces.
- **Measurement pipeline (Python, shapely/scikit-image):** strap band
  families fitted to the ink (perpendicular dark-run profiles with
  ridge-snapped centrelines — the snapping objective is the
  threshold-crossing midpoint, not the peak, because JPEG blur makes
  the darkness profile asymmetric), duplicate-band merging
  (union-find on near-collinear candidates), square end-caps trimmed
  against neighbouring bands and the raster ink, band union clipped to
  the (dilated) raster ink polygon, residual ornament ink covered by
  skeleton-traced bands; white faces = exact complement of the emitted
  ink tiles (ink pieces themselves = medial-skeleton partition of the
  ink region, skeleton branches extended along their direction until
  they exit the ink, hole-containing cells freed by thin-corridor cuts
  into simple polygons), and a final conformalization pass (every tile
  edge split at foreign vertices, zero-length edges dropped) so shared
  boundaries have identical vertex sequences. Tiles classified into
  D2 orbit classes (IoU ≥ 0.8 under the cell's point group over the
  3×3 lattice patch): the big faces fall into clean orbits (pinwheel;
  4 diagonals; 4 petals; 2 corridors; 4 corners; …) with median
  within-orbit area spread 0.13 u²; pattern colors use the two-way
  white/ink semantic (M15 precedent).
- **Verification (on the exact shipped data, Python):** sum of cell
  areas = 6930 to 3.9e-16 relative; every interior edge shared by
  exactly 2 tiles (98 border edges once, 0 interior violations); every
  interior vertex angle sum = 360° exactly (0 violations at 1822
  interior vertices); fine grid over the ±3×±3 lattice patch covered
  exactly once (27 720/27 720 sample points); 3 000-point random
  partition test zero-gap/zero-overlap. Rust: the standard trio plus
  `lattice_culling_matches_brute_force` re-emission match.
- **Implementation:** `girih_sc.rs` (feature `girih_sc`), one
  `LatticeTiling` data table — 545 cells, coordinates as 4-decimal
  literals bit-identical to the verified JSON; seed 0 = white ground
  faces, 1 = ink strap pieces. First measured-panel entry (AGENTS.md
  convention amended: decimal data tables allowed with documented
  pipeline + exact partition verification). TUI export visually
  verified cell-against-cell against the reference panel (pinwheel,
  petals, chord straps, bobbin motifs, corner structures all
  correspond).
- **Status:** committed (M35). 37 tilings, 227 tests.


> Follow-up: khatam decomposition — ✅ complete (M36)
>


### 11. Khatam girih decomposition (8-point) ✅ (M36)

- **Motif:** the classical *khatam* ({8/3} star-and-cross, "khatam" =
  the 8-pointed star of two squares, one rotated 45°) decomposed
  girih-style: the kissing arrangement (M15's `star-cross` at
  `T = 1/2`) cut into a small prototile kitab, the way classical zellige
  panels cut the star-and-cross into piece molds. Sibling of M15 (the
  undivided one-parameter family) and of the girih entries M18/M35 on
  the 10-fold side.
- **The decomposition (exact, all angles multiples of 22.5°):**
  * each {8/3} star is cut by the 8 segments joining its centre to its
    notches into **8 congruent rhombi** (45°/135°) with vertices
    centre–notch–tip–notch; every side equals the notch radius
    ρ = √(4−2√2)/4 (in lattice units, star tip radius 1/2);
  * each interstice cross (the concave 16-gon between four kissing
    stars) splits into the **central square** through the four diagonal
    star-tip apexes (side (2−√2)/2) plus **4 congruent pentagons**
    (angles 135°, 90°, 90°, 112.5°, 112.5°; four sides ρ and one square
    side), each bounded by 4 star edges and 1 square edge.
  Per unit cell: 8 rhombi + 1 square + 4 pentagons = 13 tiles; areas
  8·(√2−1)/8 + (3−2√2)/2 + 4·(1/8) = **1 exactly**.
- **Edge-to-edge across the plane:** every edge is a full edge on both
  sides (no T-junctions, no gaps); vertex figures **4⁸** at the star
  centres, **90.135.135** at the star notches, **45.45.135.135** at the
  kissing points, **45.90.112.5.112.5** at the diagonal tips — exactly
  four vertex orbits. Wallpaper group **p4m** (square lattice; the cut
  is invariant under all its mirrors).
- **Verification (Python, stdlib, ±3×±3 patch = 637 tiles):** per-cell
  area sum = 1 to 1.1e-16; every emitted tile has exactly the claimed
  sides and angles; all 448 interior edges shared by exactly 2 tiles;
  all 257 interior vertex angle sums = 360° in exactly the four orbits
  above; a 29 580-point irrational-offset grid over [−2,2]² covered
  exactly once; the patch set invariant under the p4m mirror and 90°
  rotation. All vertices built from exact Q(√2) const expressions
  (no decimals). Rust: the standard trio plus
  `lattice_culling_matches_brute_force` (list extended); suite green.
- **Implementation:** `khatam_girih.rs` (feature `khatam_girih`), one
  `LatticeTiling` — 13 cells built programmatically from the exact
  constants (`t2 = 1/4`, `tn = (√2−1)/4`, `ts = √2/4`, 45° rotation in
  Q(√2)); each shared vertex computed once so edge partners are
  bit-identical. Seeds: 0 = pentagons (largest, 1/8), 1 = square,
  2 = rhombi (pattern view: gray crosses with blue square centres, red
  rhombus stars). TUI export verified against M15's `star-cross` at
  T = 1/2: identical silhouettes, cut lines added inside.
- **Status:** committed (M36). 38 tilings, 230 tests.


### 44. Hankin motif-inference tilings ✅ (M94)

- **Method:** the *inference* half of Hankin's polygons-in-contact
  method (Kaplan dissertation §3.4) — the part that turns a substrate
  + contact angle into a finished star-pattern tiling, as opposed to
  the strapwork *layer* engine of M53–M57 (candidate 14's line
  rendering): from every substrate-edge contact, two rays lean into
  the tile (Kaplan contact angle θK measured from the edge; the
  engine uses the conjugate angle θe = 90° − θK so that the ray
  formula is `sin(θe)·t + cos(θe)·n` with t the unit edge direction
  and n the inward normal); every ray grows until it meets another
  ray; pairs are chosen greedily in order of total induced segment
  length, same-contact rays never pair (their crossing is shared),
  and colinear head-on rays join into one straight segment. The
  two-point variant splits each contact along the edge by ±δ/2 with
  the rays leaning so same-contact rays cross (dissertation Fig 3.5);
  neighbour tiles compute the same split points on shared edges, so
  motifs line up for any global δ. The paired segments are arranged
  against the substrate boundary (full planar arrangement, curves
  split at every crossing) and the bounded faces are the tiling.
- **Engine:** `hankin::build_cells` in `hankin.rs` (feature
  `hankin_ext`) — f64 port of a 40-dps mpmath prototype with the
  port-faithfulness argument measured, not assumed: greedy
  order-invariance under candidate tie permutations (the constructions
  are highly symmetric; mathematically equal costs tie exactly), with
  distinct cost classes ≥ 0.13 apart ≫ f64 noise; minimum accepted
  ray parameter 3.5e-2, minimum arrangement piece 2.5e-1, minimum
  half-edge angular gap 7.9e-1, minimum face area 6.3e-2, minimum
  canonical sort-key gap 1.8e-1 — each ≥ 7 orders above the 1e-9
  tolerances. Canonicalization (snap |v| < 1e-12 → 0, CCW,
  min-vertex-first with a 1e-12 comparison tolerance so ulp-level
  mirror ties resolve identically on both engines, faces sorted by
  (vertex count, vertices)) is applied identically in the prototype's
  transcription step and the engine, and the pattern seeds are
  congruence clusters (vertex count + area within 1e-9,
  first-appearance order). Emitted per-configuration modules
  (`hankin_4_8_8.rs`, `hankin_4_6_12.rs`, `hankin_hex.rs`) carry
  machine-transcribed constants — zero hand-copied numbers.
- **Validation battery (per configuration):** pairing complete and
  tie-invariant; per-tile face partition exact (Σ face areas = tile
  area, < 1e-25 in the prototype); 400-point planar coverage exact;
  torus fold of ONE fundamental cell's segment multiset (endpoint
  fold mod the lattice, re-wrapped after rounding) with valence
  census and closure of every strand; X-consistency at valence-4
  vertices only (valence-2 split-point bends carry no colinearity
  requirement — that IS the Kaplan semantics of Fig 3.5);
  wallpaper-group determination on the torus segment multiset
  (finite patches shed their boundary faces).
- **A. `hankin-4-8-8-rosette` (p4m, square):** 4.8.8 substrate
  (octagon + diamond), θK = 67.5° (θe = 22.5°), δ = 0 — the classic
  fourfold rosette: 8-petal rosette + core octagon in each octagon
  (9 faces), 4-pointed star + core diamond in each diamond (5
  faces); 14 cells, 4 seed orbits, per cell ⟨(1+√2,0),(0,1+√2)⟩,
  det = 3+2√2 exact in Q(√2). Torus: 4 strands × 12 segments,
  valences {4:6, 2:12}; symmetry p4m (rot90 + 2 mirrors) verified on
  the torus multiset. Matches Kaplan Fig 3.3 column 3. Not a
  duplicate of `pic-4-8-8` (that is the reflection-variant {8/2}
  midpoint-chord wall at δ = 45°; this is the greedy rosette at
  θK = 67.5° — different construction, different partition).
- **B. `hankin-4-6-12` (p6m, hexagonal):** 4.6.12 substrate
  (dodecagon + 3 squares + 2 hexagons), θK = 60° (θe = 30°), δ = 0 —
  12-petal rosette + core dodecagon (13 faces), 4-pointed stars in
  the three squares (5 each), 6-petal rosettes + core hexagons (7
  each); 42 cells, 6 seed orbits, per cell
  ⟨s(1,0), s(1/2,√3/2)⟩ with s = 3+√3, det = 9+6√3 ≈ 19.3923 exact
  in Q(√3). Torus: 12 strands × 12 segments, valences {4:18, 2:36};
  symmetry p6m (rot60 + mirror). Not a duplicate of
  `pic-3-12-12` (3.12.12 substrate; this is the true 4.6.12 — the
  dodecagon's 12-petal greedy rosette with the squares participating).
- **C. `hankin-hex-two-point` (p6m, hexagonal):** 6³ substrate (unit
  hexagon), θK = 45° (θe = 45°), δ = 0.5 two-point — the
  bracket-and-diamond weave: 12 interlaced faces around a central
  dodecagon, 13 cells, 3 seed orbits, per cell
  ⟨(3/2,√3/2),(0,√3)⟩, det = 3√3/2. Torus: 4 strands × 6 segments,
  valences {2:12}; symmetry p6m. Figure-verified against Kaplan
  Fig 3.6 row 1: (c) two-point weave (programmatic cross-correlation,
  F1 plateau 0.38–0.39 over θK ∈ [45°, 47.5°], δ ∈ [0.5, 0.58]; the
  δ = 0 weave of the same substrate matches column (b)).
- **D. `hankin-3-12-12` (p6m, hexagonal):** 3.12.12 substrate
  (dodecagon + 2 triangle orbits), θK = 45° (θe = 45°), δ = 0 — the
  shining twelve-pointed khatam: a 24-gon star core plus 12 petals
  (13 faces) in each dodecagon, a 4-pointed star with a hexagon tip
  (4 faces) in each triangle orbit; 21 cells, 4 seed orbits, per cell
  ⟨(3/2+√3, 1+√3/2), (0, 2+√3)⟩, det = 6+7√3/2 exact in Q(√3).
  θK = 45° chosen over 30°/60°: all three pair completely, but 30°
  is the flat limit (24-gon core closes at 120° tips, no points) and
  60° is spiky (60° tips) — 45° gives the classical star (90° tips,
  150° notches) at the mid-"shine" range of the §22a star heuristic
  (s = 2 for n > 6 with θ > 2π/n, i.e. θK > 30°). No 40-dps Python
  prototype for this substrate: the expected-face constants were
  machine-emitted from the Rust engine itself (self-consistency pin),
  and p6m was verified on a ±4-lattice window face multiset (1701
  faces, all 12 D6 elements, 0 mismatches at 1e-6 quantization).
  Details in §53. (M99)
- **Tests:** 19 new (4 engine tests — output equivalence against the
  machine-transcribed prototype expectations at 1e-9 per vertex for
  all three configurations, θe/θK conjugacy, engine reports; 15
  registry tests — covers/rotated-scaled/deterministic/structure
  (seed layout, Σ face areas = |det| at 1e-9)/symmetry-metadata per
  configuration) + 6 for D (1 engine self-consistency test, 5
  registry tests). Census at D: 89 → 90 tilings, 101 → 102
  memberships, 626 → 632 tests.
- **Status:** landed (M94; D landed M99).


### 53. Hankin twelvefold khatam (3.12.12) ✅ (M99)

- **Motif:** the fourth configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the first on the
  truncated-hexagonal 3.12.12 substrate and the first hankin entry
  carrying the twelvefold khatam on a substrate that also has a PIC
  strapwork twin (`pic-3-12-12`, M56). Contact angle θK = 45°
  (θe = 45°, symmetric like the two-point entry), δ = 0 single-point
  contacts: from every substrate-edge midpoint two rays lean into
  the tile at 45° to the edge, greedy min-cost pairing (Kaplan §3.4),
  planar arrangement against the tile boundary, bounded faces = the
  khatam partition.
- **Angle choice (θK ∈ {30°, 45°, 60°} probed):** all three pair
  completely on every tile class ((24 rays, 12 pairs) on the
  dodecagon, (6, 3) on each triangle orbit). θK = 30° is the flat
  limiting case — the seed-1 star core (a 24-gon) closes at 120° tips
  and the dodecagon shows no points; θK = 60° is spiky — 60° tips
  with chaotic triangle petals. θK = 45° gives the classical shining
  twelve-pointed star: 90° tips and 150° notches on the 24-gon core
  (minimum interior angle 90.0° measured on the emitted cells), the
  mid-"shine" range of the §22a star heuristic (s = 2 for n > 6 with
  θ > 2π/n, i.e. θK > 30°; θK = 45° sits mid-way between the 30°
  threshold and the spiky 60° end).
- **Cell structure:** per cell ⟨(3/2+√3, 1+√3/2), (0, 2+√3)⟩
  (dodecagon edge 1, both vectors of length 2+√3, 60° apart; the
  substrate's own lattice, `archimedean.rs::truncated_hexagonal`)
  with per-cell area 3(2+√3) + √3/2 = 6 + 7√3/2 = |det|: 13 dodecagon
  faces (12 quads + the 24-gon core), 4 + 4 triangle faces (three
  quads + one hexagon tip each orbit) — 21 cells, 4 seed orbits
  {0: 12 dodecagon petals, 1: one 24-gon core, 2: 6 triangle petals,
  3: two triangle tips}.
- **Rust:** `hankin_3_12_12()` in `hankin_3_12_12.rs`, registered
  after `hankin_hex_two_point`. Honest deviation from the M94 trio:
  no 40-dps Python prototype existed for this substrate, so the
  template tiles and EXPECTED_* constants were machine-emitted from
  the Rust engine itself — the engine battery is a self-consistency
  pin, not an independent cross-check (documented in the module
  header). 6 new tests (1 engine: face equivalence against the
  emitted arrays at 1e-9 + per-class ray/pair censuses + the θe/θK
  conjugacy pin; 5 registry: covers_region / rotated_scaled /
  deterministic / unit_structure (21 cells, seed layout, det =
  6+7√3/2, Σ areas = det at 1e-9) / symmetry_metadata).
- **Symmetry:** p6m verified position-aware on a ±4-lattice window —
  the 1701-face multiset (403 interior faces of the radius-8 disk)
  is invariant under all 12 D6 elements (rot k·60° + M_0 mirrors),
  0 key mismatches at 1e-6 quantization (keys orientation-normalized
  to survive mirror reversal and f64 noise). The sixfold centers are
  the dodecagon centers; the triangle edges sit on alternating
  dodecagon edges, mirrors at the 30°·k axes.
- **Census:** 89 → 90 tilings, 101 → 102 memberships (Islamic 12 →
  13), 626 → 632 tests.
- **Status:** landed (M99). Full bar green.


### 54. Hankin two-point square weave (4.4.4.4) ✅ (M99)

- **Motif:** the fifth configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the two-point
  (`δ = 0.5`) variant on the square 4.4.4.4 substrate. Contact angle
  θK = 45° (θe = 45°, symmetric like the two-point hex entry), two-point
  truncation δ = 0.5 edge lengths: from each edge's split pair of
  points the rays lean in at 45° to the edge and cross, greedy
  min-cost pairing (Kaplan §3.4), planar arrangement against the tile
  boundary, bounded faces = the partition.
- **Angle choice (θK ∈ {30°, 45°, 60°} probed):** all three pair
  completely on the square (8 rays, 4 pairs per tile). θK = 30° is the
  flat limiting case — the seed-2 core (an 8-gon) closes at 90° tips
  with quad petals; θK = 60° is spiky — 60° tips, nine faces per tile.
  θK = 45° gives the classical decagonal star: a 10-gon core with 90°
  tips plus two pentagonal petals and four corner triangles — the
  mid-"shine" range, symmetric θK = θE. Measured on the emitted
  cells: seed-2 core minimum interior angle 90.0° at θK = 45°
  (90.0° at θK = 30°, 60.0° at θK = 60°).
- **Cell structure:** per cell ⟨(1,0),(0,1)⟩ (unit square, det 1,
  area 1) four corner triangles (seed 0), two pentagonal petals
  (seed 1), one 10-gon star core (seed 2) — 7 cells, 3 seed orbits
  {0: 4 triangles, 1: two pentagons, 2: one 10-gon core}.
- **Rust:** `hankin_square_two_point()` in
  `hankin_square_two_point.rs`, registered after `hankin_3_12_12`.
  Honest deviations: (a) no 40-dps Python prototype for this
  substrate, so the template tile and the EXPECTED_* constants were
  machine-emitted from the Rust engine itself — the engine battery is
  a self-consistency pin, not an independent cross-check (documented
  in the module header); (b) this substrate is defined by
  `square::spec` (the Uniform family), not the Archimedean substrate
  family in `archimedean.rs`. 6 new tests (1 engine: face
  equivalence against the emitted arrays at 1e-9 + per-class
  ray/pair censuses + the θe/θK conjugacy pin; 5 registry:
  covers_region / rotated_scaled / deterministic / unit_structure
  (7 cells, seed layout, det = 1, Σ areas = det at 1e-9) /
  symmetry_metadata).
- **Symmetry:** cmm (2*22) verified position-aware — the 4-fold
  hypothesis (p4m) is falsified: 4-fold probes at the corner, edge
  midpoint, and cell-center orbits all fail, and the axis-parallel
  mirrors do not hold. The two diagonal mirror families (lines
  x − y = k and x + y = k, k ∈ ℤ) each pass on two instances; 2-fold
  points verified on-mirror at (0,0) and (0.5,0.5) (the family
  intersections) and off-mirror at (0.5,0) and (0,0.5) (edge
  midpoints). 10-element battery on radius-4 invariant disks,
  0 unexpected outcomes, 1e-6 quantization.
- **Census:** 90 → 91 tilings, 102 → 103 memberships (Islamic 13 →
  14), 632 → 638 tests.
- **Status:** landed (M99). Full bar green.


### 55. Hankin rosette-transform khatam (4.8.8, θK = 45°) ✅ (M99)

- **Motif:** first registry landing of roadmap §22. The Kaplan
  rosette transform (diss. §3.8, PROPOSED.md §22c) is a
  tiling-to-tiling operation — a Najm tiling becomes the
  corresponding Hankin tiling — and ships as the standalone module
  `rosette_transform.rs` (`hankin_ext`) that maps a substrate cell
  table (+lattice) to a transformed cell table without touching the
  M94 inference engine: regular n-gons (n ≥ 5, radius r) become an
  inner n-gon rotated π/n with radius r′ = r·cos(π/n)/(1+sin(π/n)) —
  algebraically identical to
  r′ = r(cos(π/n) − sin(π/n)tan(π(n−2)/4n)) — plus n trapezoid
  spokes, each exactly half the inner side length; irregular tiles
  (n ≤ 4) become perpendicular-bisector faces, their intersections
  clustered (Kaplan's cluster-averaging heuristic) when several
  bisector pairs coincide at one point.
- **Construction:** on the 4.8.8 substrate at θK = 45°, δ = 0 the
  octagon maps to inner octagon + 8 pentagons and each square
  (n = 4 < 5) to 4 half-squares of side 1/2 — the square's four
  bisectors all pass through its centre, only the 4 non-parallel of
  the 6 bisector pairs intersect, and all 4 intersections coincide,
  so the cluster heuristic fires (intersections = 4, clusters = 1).
  The transformed table has 93 tiles per ⟨(1+√2,0),(0,1+√2)⟩ cell —
  census 16 triangles, 36 quads, 32 pentagons, 8 hexagons, one
  16-gon (seed orbits: 8 octagon-derived groups of [4,4,4,5,5,5,5,6],
  8 inner-octagon quads + one 16-gon, 4 square-derived [3,3,3,3,4]
  groups) — and is edge-to-edge (0 interior unpaired edges, 0
  T-junctions at w = 4) with per-tile area closing to |det| = 3+2√2.
  θ = 45° chosen over 30° (`build_cells` incomplete: unpaired rays)
  and 60° (the pentagon rings collapse into a square star-web); 45°
  preserves the 32 pentagons and reads closest to Kaplan Fig 3.33(d).
  The M94 inference then runs on the transformed table unchanged
  (θE = 45°, δ = 0).
- **Verification:** 13 new tests — both r′ forms agree ≤ 1e-12 for
  n = 5…12 (identical in f64 at n = 5, 8, 12; anchors at r = 1:
  0.5095254494944288, 0.6681786379192989, 0.7673269879789604 for
  n = 5, 8, 12) with spoke = half inner side to machine precision;
  octagon → inner octagon + 8 pentagons; square → 4 half-squares
  with all 9 map points used; area census |det|; edge-to-edge at
  w = 4; fine-grid exactly-once (dual.rs house pattern);
  covers/rotated_scaled/deterministic at the house regions; per-cell
  EXPECTED_SEEDS/EXPECTED_LENS transcription machine-verified against
  the 17-digit dump; and the transformed cell table is p4m-exact to
  6.497e-16 (all 7 point-group elements about (A/2, A/2) with 9
  lattice translations; assert ≤ 1e-9).
- **Census:** 91 → 92 tilings, 103 → 104 memberships, Islamic 14 →
  15, tab bar `[11,11,15,17,12,15,5,8,6,4]`, suite 638 → 651 tests
  (landed on main after the two config-level M99 hankin entries; the
  fork's own step numbers were 89→90/101→102/626→639 on its base).
- **Status:** landed (M99) — transform module + first tiling; the
  11/13-pointed-star angle-mismatch instances ride on 22b/22e
  substrates, and 22d's θ(x) strips stay a rendering feature.


### 56. Najm template substrates — ([6,3]; 2e, 0, 3e; AC) and ([4,4]; 3e, 0, 0; A) ✅ (M99)

- **Motif:** the two substrate tilings of the Najm construction
  (Kaplan diss. §3.6.1, Fig. 3.21 / Fig. 3.22 second row), produced by
  the first **template-tiling generator** (roadmap 22b) instead of
  hand-coded coordinates. The notation `([p,q]; m_A o_A, m_B o_B,
  m_C o_C; inflation)` places a p·m_A-gon at the p-fold vertex A, a
  2·m_B-gon at the right-angle vertex B and a q·m_C-gon at the q-fold
  vertex C of the [p,q] orbifold triangle, oriented so the designated
  ray (A→AB, B→BC, C→CA) meets the named feature (v/e; features recur
  every π/n and alternate type), sized by one of four inflation cases.
  Reflecting the triangle tiles the plane; the polygons' union leaves
  the bowtie/star hole tiles.
- **Generator (`najm.rs`, feature `najm`):** `NajmSpec` (notation) →
  `radii()` (f64): case 3 (diad pair) closed form
  x = |edge|/(g_u + g_v), g = 1/(2 sin(π/n)) (vertex extent) or
  1/(2 tan(π/n)) (edge-midpoint extent); case 4 (ABC) by bisection on
  r_A (the BC gap is monotone); cases 1/2 as a sequential step list
  (touch-along-shared-edge and reach-the-opposite-edge constraints,
  minimum wins). Validation: m_B = 1 forbidden, every nonzero polygon
  sized exactly once, triangle edges must carry features. All world
  coordinates in the entries are *derived* from the solved radii by
  trig — no pasted tables.
- **Deviation from the diss. text:** the extracted case-3 formula
  reads `T(l) = T(x)/sin(π/n)`, i.e. l = x/sin(π/n), which exceeds the
  polygon's own circumradius x/(2 sin(π/n)) — Fig. 3.21 could not
  close. Independent re-derivation (and 60-digit chord-coincidence
  verification, agreement < 1e-55): **l = x/(2 sin(π/n))** vertex-type,
  **x/(2 tan(π/n))** edge-type — the text is missing a factor ½. The
  apothem reading of edge extents in cases 1/2 (l = r cos(π/n)) is
  likewise documented there.
- **`najm-63-2e-0-3e-ac` (p6m, hexagonal):** dodecagons at the 6-fold
  points of Λ = ⟨(√3,0),(√3/2,3/2)⟩ (phase 15°, chord midpoints at
  30k), enneagons at both C-orbits with **different absolute
  orientations** (phases 190° at C0 = (√3/2,½) and 250° at
  C1 = (√3,1) — the orientation rule applies per triangle copy; the
  rot180 about the 2-fold point B0 = (√3/2,0) maps one class onto the
  other translated by −a2, 190° + 180° ≡ 250° mod 40°), concave
  equilateral bowties (angles {70×4, 220×2}) at the three 2-fold
  points per cell. Case-3 AC with |AC| = 1:
  x = 2/(cot 15° + cot 20°) = 0.30866…, r_A = x/(2 sin 15°) =
  0.59629392…, r_C = x/(2 sin 20°) = 0.45123723… (cot 20° is cubic —
  this entry is floating-point, solved to < 1e-55 at 60 working
  digits). 1 doz + 2 enne + 3 bowties per cell; area census
  3r_A² + 9r_C² sin 40° + 3A_bowtie = 3√3/2 = |det| exact; vertex
  figures 150.140.70 ×12 and 140.220 ×6.
- **`najm-44-3e-0-0-a` (p4m, square):** dodecagons r_A = 1/cos 15° =
  √6−√2 (case-2 inflation on the [4,4] triangle, leg-scaled to |AB| =
  1) centered on 2Z²; the ±15° vertices land exactly on the odd grid
  lines, so horizontal/vertical neighbors share full chords through
  the 2-fold points. The interstice at (1,1) is a concave equilateral
  4-pointed star: tips on the axes at radius √3−1 (60°), reflex
  notches at the diagonal dodecagon vertices, offset 2−√3 (210°);
  side 4−2√3 = the dodecagon side. Exact in Q(√2,√3); areas 24−12√3
  + 12√3−20 = 4 = |det| exactly; vertex figures 60.150.150 ×4 and
  150.210 ×4.
- **Verification (spike `t22b_spike.py`, mpmath-60 + sympy; Rust
  `najm_tests`, 11 tests):** both tilings verified edge-to-edge
  (588/588 and 192/192 torus edge pairings), exactly-once coverage on
  130²/140² grids, area censuses closed exactly (sympy), shared-chord
  identities < 1e-55, bowtie/star equilateral + angle multisets,
  rot180 orientation witness, solver fidelity against two extra
  dissertation examples ([6,3];4e,3v,3e ABC and [4,4];3e,2e,3v;A=0.6
  sequential). Rust tests: covers/rotated/deterministic per entry,
  radii vs emitted f64 constants (1e-14), regularity/equilateral/
  angle checks, area census vs |det|, primitive-lattice witness
  (dodecagon centers = exactly the declared lattice over a generated
  patch). M76 symmetry-legality test passes both bases (p6m/Hexagonal
  via the 60° equal pair, p4m/Square via the orthonormal pair).
- **Status:** landed (M99). 89 → 91 tilings, 101 → 103 memberships,
  Islamic 12 → 14, 626 → 637 tests. The generator covers Euclidean
  [4,4]/[6,3] instances; hole decomposition is per-entry (22a/22c
  consume substrates), spherical/hyperbolic instances belong to 22f.


### 57. Hankin sixfold hexagram (3.6.3.6) ✅ (M100)

- **Motif:** the sixth configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the first on the
  trihexagonal 3.6.3.6 substrate (unit flat-top hexagons plus two
  equilateral triangles per cell, lattice ⟨(2,0),(1,√3)⟩, det 2√3).
  Single-point contacts (δ = 0), contact angle θK = 60° (θe = 30°):
  rays from each hexagon edge midpoint lean in at 60° to the edge,
  greedy min-cost pairing (Kaplan §3.4), planar arrangement against
  the tile boundary.
- **Angle choice (θK ∈ {30°, 45°, 60°, 67.5°, 90°} probed):** the
  hexagon tile pairs completely at every probed angle (12 rays, 6
  pairs); the two equilateral triangle tiles pair with nothing (6
  rays, 0 pairs — their contact rays never cross for θK < 90°), so
  their motif stays the original single-face triangle. Measured
  minimum interior angle of the hexagon's central core face: θK =
  30° → 120.0° (a plain hexagon core — no star); θK = 45° → 90.0°
  (truncated-hexagon 12-gon — no star); **θK = 60° → 60.0° (the
  classical regular {6/2} hexagram: a 12-gon core with 60° tips and
  240° notches, vertices alternating on circles of radius 1/2 and
  √3/2, numerically verified)**; θK = 67.5° → 45.0° (a sharper star
  variant — probed, not registered); θK = 90° is degenerate (the
  star core collapses; the hexagon splits into 6 quadrilaterals
  plus a central equilateral triangle — rejected). 60° is chosen:
  the largest minimum interior angle among the star candidates (no
  razor-thin slivers) and the regular hexagram. Per the §22a star
  heuristic (s = 2 when n > 6 and θ > 2π/n), n = 6 is not a
  formal star-trigger polygon: the hexagon star first appears at the
  critical angle 60° = 2π/6, whose regular {6/2} endpoint is this
  configuration.
- **Cell structure:** 9 faces per lattice cell — 6 lozenge rhombi
  plus the 12-gon hexagram core from the hexagon class (7 faces,
  seed 0), one unpaired triangle face each from classes 1 and 2.
  Per-class face / ray / pair censuses {7,1,1} / {12,6,6} /
  {6,0,0}; per-class completeness {true,false,false} — the
  whole-cell report is therefore not "complete" and the engine test
  asserts per-class completeness instead.
- **Rust:** `hankin_3_6_3_6()` in `hankin_3_6_3_6.rs`, registered
  after `hankin_square_two_point`. Honest deviation: no 40-dps
  Python prototype for this substrate — the template tiles and the
  EXPECTED_* constants were machine-emitted from the Rust engine
  itself, so the engine battery is a self-consistency pin, not an
  independent cross-check (documented in the module header).
  Distinct from `pic-3-6-3-6`: same 3.6.3.6 substrate, different
  construction (greedy inference vs. the δ = 60° reflection-chord
  strapwork rule) and a different partition. 6 new tests (1 engine
  self-consistency: face equivalence against the emitted arrays at
  1e-9 + per-class ray/pair censuses + the θe/θK conjugacy pin; 5
  registry: covers_region / rotated_scaled / deterministic /
  unit_structure (9 cells, seed layout {0×6, 1, 2×2}, det = 2√3,
  Σ areas = det at 1e-9) / symmetry_metadata).
- **Symmetry:** p6m verified position-aware — the ±5-lattice window
  (1089 faces) with the radius-8 interior disk (517 faces) is
  invariant under all 12 D6 elements (rot k·60° + M_0 mirrors), 0
  key mismatches at 1e-6 quantization. (The ±4 window with R = 6 or
  8 shows 1–2 mirror mismatches — a boundary artifact; ±4 with
  R ∈ {4,5} is clean, 127/199 faces.)
- **Census:** 91 → 92 tilings, 103 → 104 memberships (Islamic 14 →
  15), 638 → 644 tests.
- **Status:** landed (M100). Full bar green.


### 56. Hankin sixfold rhombitrihex motif (3.4.6.4) ✅ (M100)

- **Motif:** the seventh configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the first on the
  rhombitrihexagonal 3.4.6.4 substrate (unit pointy-top hexagons
  plus 3 unit squares and 2 equilateral triangles per lattice cell
  ⟨(1.5+√3/2, √3/2+1/2),(0,1+√3)⟩, det 4√3/2+3 exact in Q(√3)).
  Single-point contacts (δ = 0), contact angle θK = 60° (θe =
  30°): rays from each edge midpoint lean in at 60° to the edge,
  greedy min-cost pairing (Kaplan §3.4), planar arrangement against
  the tile boundary.
- **Angle choice (θK ∈ {30°, 45°, 60°, 67.5°, 90°} probed):** the
  hexagon tile pairs completely at every probed angle (12 rays, 6
  pairs); the three square tiles pair with nothing for θK ∈
  {30°, 45°, 60°, 67.5°} (8 rays, 0 pairs — at 45° the
  adjacent-edge rays are exactly parallel or meet exactly at an
  edge midpoint, and the engine rejects zero-length crossings; at
  90° they pair head-on into 2 pairs / 2 faces), so their motif
  stays the original square. The two equilateral triangle tiles:
  at θK = 30°/45°/67.5° they carry a proper three-pointed star
  (6 rays, 3 pairs, 4 quadrilateral faces) — but the two
  instances break congruence (the greedy tie-break gives the two
  orientations different motifs, one a 6-gon, the other 4-gons,
  which is not D6-compatible); at θK = 60° all three pairs sit
  exactly on the edge midpoints: the star collapses into the
  classical 4-subdivision (four side-1/2 equilateral triangles
  each — orientation-invariant, both instances congruent); at
  θK = 90° the triangle pairs head-on into 2 pairs / 2 faces and
  the hexagon's star core collapses (6 faces instead of 7).
  Measured minimum interior angle over all 18 cell faces: 30.0°
  at θK = 30° (the hexagon splits into six thin sliver triangles),
  45.0° at 45°, **60.0° at 60° (no slivers — the regular
  hexagram tips and the side-1/2 equilateral triangles)**, 45.0°
  at 67.5° (sharper hexagram, 45° tips), degenerate at 90° (no
  hexagon star; 13 faces). 60° is chosen: the most classical
  sixfold star on this substrate — a regular {6/2} hexagram per
  hexagon (12-gon core with 60° tips and 240° notches, vertices
  alternating on circles of radius 1/2 and √3/2, numerically
  verified) — and the largest minimum interior angle among the
  star candidates (60° at 60° vs 45° at 67.5°). It is also the
  only probed angle whose all-motif configuration is
  orientation-invariant (D6-compatible).
- **Cell structure:** 18 faces per lattice cell — 6 lozenge
  rhombi plus the 12-gon hexagram core from the hexagon (7 faces,
  seed 0 / seed 1), the three unpaired squares (seed 2), and 8
  side-1/2 equilateral triangles from the two triangle tiles (4
  each, seed 3). Per-instance face / ray / pair censuses
  [7,1,1,1,4,4] / [12,8,8,8,6,6] / [6,0,0,0,3,3]; per-instance
  completeness [true,false,false,false,true,true] — the whole-cell
  report is therefore not "complete" and the engine test asserts
  per-instance completeness instead.
- **Rust:** `hankin_3_4_6_4()` in `hankin_3_4_6_4.rs`, registered
  after `hankin_3_6_3_6`. Honest deviation: no 40-dps Python
  prototype for this substrate — the template tiles and the
  EXPECTED_* constants were machine-emitted from the Rust engine
  itself (the angle choice was likewise made on the Rust engine's
  own emitted cells), so the engine battery is a self-consistency
  pin, not an independent cross-check (documented in the module
  header). Distinct from `rhombitrihexagonal`: same 3.4.6.4
  substrate, different construction (greedy inference vs. the
  Archimedean tiling) and a different partition. 6 new tests (1
  engine self-consistency: face equivalence against the emitted
  arrays at 1e-9 + per-instance ray/pair censuses + the θe/θK
  conjugacy pin; 5 registry: covers_region / rotated_scaled /
  deterministic / unit_structure (18 cells, seed layout {0×6, 1,
  2×3, 3×8}, det = 4√3/2+3, Σ areas = det at 1e-9) /
  symmetry_metadata).
- **Symmetry:** p6m verified position-aware — the ±5-lattice
  window (2178 faces) with the radius-8 interior disk (553 faces,
  319 at R = 6) about a hexagon center is invariant under all 12
  D6 elements (rot k·60° + M_0 mirrors — the 90° + 60k mirror
  families of this tiling, since the squares sit on the 30°/150°/
  270° hexagon edges), 0 key mismatches at 1e-6 quantization.
- **Census:** 92 → 93 tilings, 104 → 105 memberships (Islamic 15 →
  16), 644 → 650 tests.
- **Status:** landed (M100). Full bar green.


### 63. Hankin elongated-triangular motif (3.3.3.4.4) ✅ (M107)

- **Motif:** the eighth configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the first on the
  elongated-triangular 3.3.3.4.4 substrate (alternating rows of
  unit equilateral triangles and unit squares, cmm; one lattice
  cell ⟨(1,0),(1/2,1+√3/2)⟩ = two triangles + one square, det
  1+√3/2 exact in Q(√3)). Single-point contacts (δ = 0), contact
  angle θK = 60° (θe = 30°): rays from each edge midpoint lean in
  at 60° to the edge, greedy min-cost pairing (Kaplan §3.4), planar
  arrangement against the tile boundary. The down-triangle instance
  runs CCW in the engine's table: the archimedean module stores it
  CW, and a CW tile emits outward-pointing rays that pair with
  nothing at every probed angle (1 face, 0 pairs), so the all-CCW
  in-cell table is the one that produces a paired motif.
- **Angle choice (θK ∈ {30°, 45°, 60°, 67.5°, 90°} probed, on the
  all-CCW in-cell table):** the two triangle tiles pair completely
  at 30°–67.5° (6 rays, 3 pairs, 4 faces — measured minimum
  interior angle 30.0° / 45.0° / 60.0° / 45.0°; at 90° the 3
  spokes meet at the incenter and the pairing degenerates to
  2 pairs / 2 faces, incomplete). The square pairs completely at
  30°–67.5° (8 rays, 4 pairs, 5 faces — four corner quadrilaterals
  + central octagon); at 90° the star collapses to 4 faces. The
  CW square and CW down-triangle never pair (outward rays, 0 pairs
  at every probed angle — documented reject). 60° is chosen: the
  largest minimum interior angle among the complete candidates
  (60° — both triangles sit in the classical side-1/2
  4-subdivision limit, orientation-invariant, both instances
  congruent).
- **Cell structure:** 13 faces per lattice cell — 8 side-1/2
  equilateral triangles (area √3/16 each, seed 0 — both
  subdivision orbits share the (3-gon, √3/16) seed signature), 4
  square corner quadrilaterals (area (3−√3)/8 each, seed 1), 1
  central octagon (area (√3−1)/2, seed 2); Σ = 1+√3/2 = |det|
  exact. Per-instance face/ray/pair censuses [4,4,5] / [6,6,8] /
  [3,3,4]; all three instances pair completely, so the whole-cell
  report is "complete". All face vertices lie in Z[1/24] +
  Z[1/48]·(√3/2) (a/24 + b√3/96, (a,b) ∈ ℤ² unique), verified
  exactly on a ±9-lattice window of 4693 faces.
- **Rust:** `hankin_3_3_3_4_4()` in `hankin_3_3_3_4_4.rs`,
  registered after `hankin_3_4_6_4`. Honest deviation: no 40-dps
  Python prototype for this substrate — the template tiles and the
  EXPECTED_* constants were machine-emitted from the Rust engine
  itself (the angle choice and the θK survey were made on a faithful
  f64 port of the engine plus an exact Q(√3) symmetry battery), so
  the engine battery is a self-consistency pin, not an independent
  cross-check (documented in the module header). Distinct from
  `elongated-triangular`: same 3.3.3.4.4 substrate, different
  construction (greedy inference vs. the Archimedean tiling) and a
  different partition. 6 new tests (1 engine self-consistency: face
  equivalence against the emitted arrays at 1e-9 + per-instance
  ray/pair censuses + the θe/θK conjugacy pin; 5 registry:
  covers_region / rotated_scaled / deterministic / unit_structure
  (13 cells, seed layout {0×8, 1×4, 1×2}, det = 1+√3/2, Σ areas =
  det at 1e-9) / symmetry_metadata (cmm, centered rectangular,
  Islamic)).
- **Symmetry:** cmm verified exactly (exact Q(√3) face-maps —
  faces re-canonicalized to CCW / minimum-vertex before keying,
  rigid images re-canonicalized the same way; the 551-face disk
  about the square center C = (1, H+1/2) with adaptive R² =
  24882743452934203698152347053/10²⁶ — two face centroids sit at
  exactly distance 5, so the probe radius is the strict midpoint of
  the nearest in/out squared distances): all 20 cmm elements
  0-mismatch on the face multiset — translations ⟨a1 = (1,0),
  a2 = (1/2,1+H)⟩; two-fold orbits {C+L} ∪ {C+a1+L} ∪
  {(1/2,H+1/2)+L} (on the mirror lines) ∪ {(±1/4,H/2)+L} ∪
  {(1/4,H/2+1+H)} (on the glides); mirrors x = k/2 (all k) and
  y = −1/2+k(1+H); glides y = H/2+k(1+H) of shift a1/2. Negative
  controls fail: rot90 about C (exact), rot30/rot60 about the
  triangle centroids, the off-orbit mirror y = H/2, the off-orbit
  glides. The substrate's own cmm group survives the hankin
  partition.
- **Census:** 100 → 101 tilings, 112 → 113 memberships (Islamic 19
  → 20), 690 → 696 tests.
- **Status:** landed (M107, fork-local numbering — renumbered at
  integration). 101 tilings, 113 memberships, 696 tests. Full bar
  green: cargo check + clippy (both target sets) zero warnings,
  cargo test --workspace --all-features 696/696.

### 66. Hankin snub-square motif (3.3.4.3.4) ✅ (M111)

> Numbering note: numbered §66 on the fork (M111, fork-local
> numbering — provisional); renumbered at integration by landing
> order (as §63 was the fork's provisional §65 and §64 the fork's
> provisional §62).

- **Motif:** the ninth configuration of the M94 inference engine
  (`hankin::build_cells`, feature `hankin_ext`) — the first on the
  snub-square 3.3.4.3.4 substrate (the snub of the square tiling: 4
  unit squares in two orientation classes + 8 unit triangles per
  cell, p4g; one lattice cell ⟨(1+√3,0),(0,1+√3)⟩, det (1+√3)² =
  4+2√3 exact in Q(√3)). Single-point contacts (δ = 0), contact
  angle θK = 60° (θe = 30°): rays from each edge midpoint lean in
  at 60° to the edge, greedy min-cost pairing (Kaplan §3.4), planar
  arrangement against the tile boundary. All twelve instances are
  stored CCW in their in-cell positions in the archimedean module;
  the CW pitfall probed in the M107/M111 survey still holds — a
  reversed instance emits rays that point out of the tile interior
  and pair with nothing at every probed angle (1 face, 0 pairs),
  so the all-CCW in-cell table is the one that produces a paired
  motif.
- **Angle choice (θK ∈ {30°, 45°, 60°, 67.5°, 90°} probed, on the
  all-CCW in-cell table):** the four square tiles pair completely
  at 30°–67.5° (8 rays, 4 pairs, 5 faces — measured minimum
  interior angle 30.0° / 45.0° / 60.0° / 45.0°; at 90° the square
  star collapses to 4 faces). The eight triangle tiles pair
  completely at 30°–67.5° (6 rays, 3 pairs, 4 faces — minimum
  30.0° / 45.0° / 60.0° / 45.0°; at 90° the 3 spokes meet at the
  incenter and the pairing degenerates to 2 pairs / 2 faces,
  incomplete). All twelve CW rows never pair (outward rays, 0
  pairs at every probed angle — documented reject). 60° is
  chosen: the largest minimum interior angle among the complete
  candidates (60° — each square in its 5-face motif, each
  triangle in the classical side-1/2 4-subdivision limit).
- **Cell structure:** 52 faces per lattice cell = 4 square tiles ×
  5 faces (four corner quadrilaterals + a central octagon) + 8
  triangle tiles × 4 faces (side-1/2 4-subdivision): 16 square
  corner quadrilaterals (area (3−√3)/8 each, seed 0), 4 central
  octagons (area (√3−1)/2 each, seed 1), 32 side-1/2 equilateral
  triangles (area √3/16 each, seed 2); Σ = 4+2√3 = |det| exact.
  Per-instance face/ray/pair censuses [5,5,5,5,4,4,4,4,4,4,4,4] /
  [8,8,8,8,6,6,6,6,6,6,6,6] / [4,4,4,4,3,3,3,3,3,3,3,3]; all
  twelve instances pair completely. All face vertices lie in
  (ℤ+ℤ√3)/4, verified exactly in the full motif field Q(√2,√3):
  the √2 and √6 coefficients are identically zero (maximum
  coefficient denominator 4) — √2 enters only as the direction of
  the 45° motif/glide rays, no face coordinate carries it.
- **Rust:** `hankin_3_3_4_3_4()` in `hankin_3_3_4_3_4.rs`,
  registered after `hankin_3_3_3_4_4`. Honest deviation: no 40-dps
  Python prototype for this substrate — the expected-face
  constants were machine-emitted by a faithful f64 port of the Rust
  engine (the M111 spike) and cross-audited exactly in the
  Q(√2,√3) coefficient ring, so the engine battery is a
  self-consistency pin, not an independent cross-check (documented
  in the module header). Distinct from `snub-square`: same 3.3.4.3.4
  substrate, different construction (greedy inference vs. the
  Archimedean tiling) and a different partition. 6 new tests (1
  engine self-consistency: face equivalence against the emitted
  arrays at 1e-9 + per-instance face/ray/pair censuses + the
  θe/θK conjugacy pin; 5 registry: covers_region / rotated_scaled
  / deterministic / unit_structure (52 cells, seed layout
  {0×16, 1×4, 2×32}, det = 4+2√3, Σ areas = det at 1e-9) /
  symmetry_metadata (p4g, square, Islamic)).
- **Symmetry:** p4g verified exactly (exact Q(√2,√3) face-maps —
  faces re-canonicalized to CCW / minimum-vertex before keying,
  rigid images re-canonicalized the same way; on the 121-cell
  window of 6292 faces, all distinct, each candidate censused
  0-mismatch on its interior square S(c, 2L)): translations (L,0)
  / (0,L) with L = 1+√3 and the centered (L/2, ±L/2) — 788 faces
  each, 0 mismatch; (L/2, 0) / (0, L/2) fail 788/788 — so the
  face-set translation lattice is the CENTERED lattice
  Λ′ = ⟨(L/2,L/2),(L/2,−L/2)⟩, not L² (conventional p4g cell
  side L/√2). Four-fold centers (769 faces each, two Λ′ orbits of
  two points mod L²): {(L/4, L/4+1/2), (L/4, 3L/4+1/2),
  (3L/4, L/4+1/2), (3L/4, 3L/4+1/2)} ≈ {(0.683, 1.183),
  (0.683, 2.549), (2.049, 1.183), (2.049, 2.549)}. Two-fold
  centers (800 faces each, two Λ′ orbits of two points mod L²):
  the four 4-fold points ∪ {(0, 1/2), (0, (L+1)/2), (L/2, 1/2),
  (L/2, (L+1)/2)} ≈ {(0, 0.5), (0, 1.866), (1.366, 0.5),
  (1.366, 1.866)}. Glides, measured off the L/8 grid the window
  sweep cannot reach: 45° lines y−x ≡ L/4+1/2, 3L/4+1/2 (mod L)
  of shift ±(L/4, L/4), and 135° lines y+x ≡ L/4+1/2, 3L/4+1/2
  (mod L) of shift ±(L/4, −L/4) (785 faces each, 0 mismatch).
  The full-disk census about (5L/4, 5L/4) (2615 faces, adaptive
  exact R² = 377/6 + 98√3/3 ≈ 119.413660 — the strict midpoint of
  the nearest in/out squared centroid distances) confirms the
  orbit structure. Negative controls fail: no mirror line in
  either family (all 18 t = 0 controls, 772–800 faces tested),
  the half-period shifts ±(L/2, ±L/2) on the off-grid lines (their
  squares are the passing centered-lattice translations), rot45
  about a 4-fold center, and the off-orbit L/8 mirrors and glides
  — p4g (glides, no mirrors), consistent with the substrate
  group; the substrate's own p4g survives the hankin partition.
- **Census:** 109 → 110 tilings, 121 → 122 memberships (Islamic 20
  → 21), 729 → 735 tests (fork-local numbers; renumbered at
  integration).
- **Status:** landed on the hankin-snub fork (M111, fork-local
  numbering — renumbered at integration). 110 tilings, 122
  memberships, 735 tests. Full bar green: cargo check + clippy
  (both target sets) zero warnings, cargo test --workspace
  --all-features 735/735.

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`star-cross`](../tilings/star-cross.md) — Star and Cross ({8/3})
- [`hexagram`](../tilings/hexagram.md) — Hexagram and Hexagon ({6/2})
- [`girih-shesh-band`](../tilings/girih-shesh-band.md) — Girih Shesh-Band (6.6.6)
- [`girih-star-cross`](../tilings/girih-star-cross.md) — Girih Star-and-Cross (periodic)
- [`girih-w85`](../tilings/girih-w85.md) — Girih 12-pointed star interlace (W85)
- [`pic-4-8-8`](../tilings/pic-4-8-8.md) — Hankin PIC fourfold khatam
- [`pic-3-6-3-6`](../tilings/pic-3-6-3-6.md) — Hankin PIC sixfold khatam
- [`pic-3-12-12`](../tilings/pic-3-12-12.md) — Hankin PIC twelvefold khatam
- [`hankin-4-8-8-rosette`](../tilings/hankin-4-8-8-rosette.md) — Hankin rosette khatam (4.8.8)
- [`hankin-4-6-12`](../tilings/hankin-4-6-12.md) — Hankin dodecagonal rosette (4.6.12)
- [`hankin-hex-two-point`](../tilings/hankin-hex-two-point.md) — Hankin two-point hex weave (6³)
- [`hankin-3-12-12`](../tilings/hankin-3-12-12.md) — Hankin twelvefold khatam (3.12.12)
- [`hankin-3-6-3-6`](../tilings/hankin-3-6-3-6.md) — Hankin sixfold hexagram (3.6.3.6)
- [`hankin-3-4-6-4`](../tilings/hankin-3-4-6-4.md) — Hankin sixfold rhombitrihex motif (3.4.6.4)
- [`hankin-square-two-point`](../tilings/hankin-square-two-point.md) — Hankin two-point square weave (4.4.4.4)
- [`hankin-4-8-8-rosette-transform`](../tilings/hankin-4-8-8-rosette-transform.md) — Hankin rosette-transform khatam (4.8.8)
- [`hankin-3-3-3-4-4`](../tilings/hankin-3-3-3-4-4.md) — Hankin elongated-triangular motif (3.3.3.4.4)
- [`hankin-3-3-4-3-4`](../tilings/hankin-3-3-4-3-4.md) — Hankin snub-square motif (3.3.4.3.4)
- [`khatam-girih`](../tilings/khatam-girih.md) — Khatam Girih Decomposition (8-point)
- [`najm-63-2e-0-3e-ac`](../tilings/najm-63-2e-0-3e-ac.md) — Najm template (6,3)
- [`najm-44-3e-0-0-a`](../tilings/najm-44-3e-0-0-a.md) — Najm template (4,4)
- [`girih-bowtie`](../tilings/girih-bowtie.md) — Girih Bowtie (72.72.216)
- [`girih-decagon`](../tilings/girih-decagon.md) — Girih Decagon (10.6.10)
- [`girih-pentagon`](../tilings/girih-pentagon.md) — Girih Pentagon (5.5.4.4)
- [`girih-rhombus`](../tilings/girih-rhombus.md) — Girih Rhombus (72.72.108.108)
- [`girih-quasiperiodic`](../tilings/girih-quasiperiodic.md) — Girih Star-and-Cross (quasiperiodic)
- [`andritz-bowtie-hexagon`](../tilings/andritz-bowtie-hexagon.md) — Andritz Bowtie-and-Hexagon (quasiperiodic)
- [`andritz-bowtie-hexagon-10gon-1`](../tilings/andritz-bowtie-hexagon-10gon-1.md) — Andritz 10gon-1 (quasiperiodic)
- [`andritz-bowtie-hexagon-10gon-2`](../tilings/andritz-bowtie-hexagon-10gon-2.md) — Andritz 10gon-2 (quasiperiodic)
- [`andritz-bowtie-hexagon-10gon-3`](../tilings/andritz-bowtie-hexagon-10gon-3.md) — Andritz 10gon-3 (quasiperiodic)

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
