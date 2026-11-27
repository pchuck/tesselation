# Isohedral types family

The parameterized isohedral (IH) types — Grünbaum–Shephard's IH1–IH93
monohedral tiling types realized as incidence symbols (tile symbol +
adjacency symbol) over their topologies, with shaped edges as exact
per-orbit polylines so tiles stay polygons and the coverage contracts
hold. The "Escher arc" (PROPOSED.md §13); design doc:
[docs/isohedral-generator-design.md](../isohedral-generator-design.md).

Code: `crates/tess-tilings/src/families/isohedral/` — cargo feature
`isohedral` (default-on).
Adding a tiling here touches only this family: `families/isohedral.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (5 tilings)

| id | IH type | topology | group | notes |
|---|---|---|---|---|
| `ih1-p1-hexagon` | IH1 | 6³ honeycomb (dual: triangular) | p1 | unconstrained hexagon; 3 freeform (J) edge orbits; 4 shape params; 1 aspect |
| `ih6-pgg-hexagon` | IH6 | 6³ honeycomb (dual: triangular) | pgg | 2 rotation-paired (U) + 2 freeform (J) orbits; 5 shape params; 4 aspects |
| `ih7-p3-hexagon` | IH7 | 6³ honeycomb (dual: triangular) | p3 | 3 freeform (J) orbits with showpiece curves; record in Q(√3); 2 shape params; 3 aspects |
| `ih27-pgg-pentagon` | IH27 | Cairo V3.3.4.3.4 (dual: snub square) | pgg | pentagon; 1 U + 2 J orbits; 3 shape params; 4 aspects; isohedral variant — deliberate edge deformation, not a pentagon type |
| `ih29-p4g-pentagon` | IH29 | Cairo V3.3.4.3.4 (dual: snub square) | p4g | pentagon; Klein-paired base orbit (straight edge) + J orbit; 1 shape param; 4 aspects; isohedral variant — deliberate edge deformation, not a pentagon type |

All five are 1-isohedral realizations pinned at exact rational parameter
values (IH7's fitted coefficients are the `Q(√3)` ones); the engine
also accepts arbitrary float parameters — off-default floats are
quantized to the 2⁻²⁰ grid and handled exactly there
(`engine.rs::resolve_params`).
The pentagon templates sit on the Cairo net; they are isohedral
variants — deliberate edge deformations, not pentagon types — with a
Laves secondary; the family never touches the Wallpaper tab.

## Per-tiling documents

- [`ih1-p1-hexagon`](../tilings/ih1-p1-hexagon.md) — IH1, p1, generic hexagon on the 6³ net
- [`ih6-pgg-hexagon`](../tilings/ih6-pgg-hexagon.md) — IH6, pgg, U/J hexagon on the 6³ net
- [`ih7-p3-hexagon`](../tilings/ih7-p3-hexagon.md) — IH7, p3, showpiece hexagon in Q(√3)
- [`ih27-pgg-pentagon`](../tilings/ih27-pgg-pentagon.md) — IH27, pgg, pentagon on the Cairo net
- [`ih29-p4g-pentagon`](../tilings/ih29-p4g-pentagon.md) — IH29, p4g, pentagon with a straight base edge

## Build arcs

### 1. Isohedral (IH) generator + five showcase instances ✅ (M162)

- **Motif:** Grünbaum–Shephard's IH1–IH93 monohedral tiling types realized as incidence symbols (tile symbol + adjacency symbol) over their topologies, with shaped edges as exact per-orbit polylines so tiles stay polygons and the coverage contracts hold (`ih1-p1-hexagon`, `ih6-pgg-hexagon`, `ih7-p3-hexagon`, `ih27-pgg-pentagon`, `ih29-p4g-pentagon`; family isohedral; M162 — PROPOSED.md §13, the "Escher arc"; Isohedral primary / Laves secondary on the two pentagons (isohedral variants — deliberate edge deformations, not pentagon types)).
- **Key facts:** a Kaplan/Tactile incidence-symbol engine over the 11 Laves topologies — vertex/translation/aspect linear forms as engine data, per-edge (fl, ro) orientation flags, adjacency symbol derived by exact geometric edge matching (BFS-closure verified in the spike, never hand-written); placement T_{a,i,j} = translate(i·t1 + j·t2) ∘ aspects[a] — a search-free (i,j)-box × aspects emission; all geometry in an exact Q(√3) field (canonical re + sq·√3, const-gcd i128 rationals) so one canonical f64 rounding per emitted coordinate makes every shared edge bit-identical, float params quantized to the 2⁻²⁰ grid and handled exactly there. Five instances: IH1 p1 (3×J), IH6 pgg (2×U + 2×J), IH7 p3 (3×J — the Q(√3) record) hexagons on the 6³ net; IH27 pgg, IH29 p4g pentagons on the Cairo net (IH29's Klein-mixed straight edge; D-census asserts ROT 2 + ROFL 2 literally). The arc's hard-won fix (design doc §9.1): the engine initially read Tactile's 2×3 aspect matrix transposed — p1/pgg's symmetric linear parts hide a transposition; p3/p4g rotations exposed it (IH7/29 failed coverage, aspect-symmetry, D-census, shared-edge); fixed against spike2.build_exact + tiling.cpp, first surfaced by the irrational-offset grid. Deviations: IH21 dropped (irreducible decimal coefficients — cannot pin exactly); pentagon_type cross-reference left None (classical 1–15 mapping unverified offline); `isohedral: Some(1)` = one isohedral class (classify.rs semantics), the IH type number lives in `method` text. Full technique story — incidence symbols as engine data, the exact Q(√3) field, the single-rounding bit-identity policy, the float-parameter quantization contract, the per-spike and per-instance verification evidence — is told in [docs/isohedral-generator-design.md](../isohedral-generator-design.md), whose §9/§9.1 carry the per-spike and per-instance verification evidence.
