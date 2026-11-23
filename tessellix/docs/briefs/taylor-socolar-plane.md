# Brief: taylor-socolar plane-filling presentation (follow-up to M86/M86-fix)

> **Status (2026-11-23):** user directive — "where applicable, fill the
> plane." M86 landed the paper's **forced finite patch** (529 hexagons,
> Fig. 18 of the Socolar–Taylor paper) as a verbatim state table; the
> paper's actual tiling is plane-filling (CHT = "centered hexagonal
> tiling"), so the shipped island presentation is a generator-presentation
> choice, not mathematics. This brief upgrades the presentation to the
> full plane. **Census is UNCHANGED** (same registry id, same family
> count, same built-ins) — a semantics/coverage upgrade, not a new
> tiling. The construction below is PINNED (spike trail in §Pinned
> evidence); the worker implements and certifies it — no re-derivation.

**Family:** `aperiodic` · **Module:**
`crates/tess-tilings/src/families/aperiodic/taylor_socolar.rs`
(2678 ln — read ALL of it first; the 8 convex-piece TEMPLATES, DELTA,
SIGMA_SUM, sigma, color_red, and the R1/R2/R3 handoff battery at lines
~2174–2229 are reused verbatim) · **Precedent to mirror:**
`docs/briefs/glb-binary-n7-plane.md` (plane-presentation upgrade shape;
structural difference: TS tiles are all unit hexagons — no supertiles,
no eps-regions; "meshing" = decoration consistency = the R1/R2/R3
census, which the module already has) · **Durable derivation record:**
`.scratch/taysoc-plane-recon/NOTES.md` (spike trail S1–S7; the numbers
below are its certified outputs).

### Goal

`Tiling::generate` on `taylor-socolar` must return **all unit tiles
intersecting any query bbox** — the tess-core contract over the whole
plane, up to the MAX_TILES budget. Today `generate` serves the finite
529-site rhombus table and the test `taylor_socolar_empty_beyond_patch`
(L~2163) enforces the island contract; that contract flips to plane
semantics with this brief. Target state assignment (all numbers
measured, §Battery): the pinned closed-form construction in §Pinned
construction reproduces the shipped table **529/529** and gives
**zero** R1/R2/R3 violations on hexagonal windows R=6/8/10/14 — a
genuine plane tiling (the paper's CHT, the specific member that
contains Fig. 18), not an island.

### Literature basis (do NOT re-derive; cited facts are certified)

Socolar & Taylor, "An aperiodic hexagonal tile" (JCTA; arXiv:1008.3103
per module/docs — the local paper extract is headed [1003.4279]; same
paper, one of the two IDs is a miscite — **flag in the report, do not
edit the citation without verification**). Key lines (extract
`ap_paper.txt`, verbatim-verified):

- **§2 (plane existence, lines 49–51):** hierarchy of interpenetrating
  honeycombs at lattice constants 2ⁿa, φ₁=3/4, ηₙ=4⁻ⁿ→0 ("the portion
  of the plane that does not get tiled ... is at most a set of measure
  zero"); "we insist that the lattice at each level is placed so as
  always to leave a designated central hexagon empty ... for any tile
  not lying on one of twelve infinite spokes ... the completion of
  level n completely determines the tile decoration ... specify an
  arbitrary orientation of the decoration of the central hexagon and
  fill in the spokes to match that choice." Fig. 7 caption: spokes at
  60° multiples from the horizontal (black-stripe) and 60° from the
  vertical (red–blue). "We refer to the tiling ... as the 'centered
  hexagonal tiling' (CHT)"; CHT minus spokes: reflection through any
  spoke + 60° rotation invariant.
- **§3 (Thm 2/3, lines 58–59):** every finite configuration repeats
  periodically with lattice constant 2^m a (homogeneity); a
  single-color vertex is forbidden by R3 (the second LI class is
  excluded ⇒ substitution-generated tilings are CHT-class only).
- **§5 (closed form, Lemmas 7/8 + Thm 5, lines 261–262):**
  Q(n)=gcd(2ⁿ,n)=2^{v2(n)}, Q(0)=∞. Panel decomposition: tile =
  i·eₙ + j·e_{1+mod3(n+1)}, panel n∈{1,2,3} iff Q(|i|)=Q(|j|).
  pBS = mod₂⌊i/Q(|i−j|)⌋ (Lemma 7, i−j≠0), pRB = mod₂⌊i/Q(|i+j|)⌋
  (Lemma 8, i+j≠0), SIGNED i, "for i<0 the pattern is clearly
  continued as desired" (floor division — Rust: `i.div_euclid(q)`);
  parity p = pBS^pRB (Thm 5). Line 261: the i/Q(0)≡0/1 convention
  values on the twelve spokes are "S=(0,1,1,0,1,0) and s=(0,1,0,1,1,0)
  or (1,1,0,0,1,0)" (spoke-indexing of the paper's Figs. 21 family).
  Line 262: "the parity sequence along each of the twelve rays ... is
  the regular paper folding sequence 110110011100100… or its
  complement"; for i−j=0 the parity is mod₂⌊i/Q(|2i|)⌋ "equivalent to
  the specification of the paper folding sequence A014577, as encoded
  in the program by Somos for the related sequence A091072."
- **Fig. 18 caption:** "A coloring of the forced 2D tiling with tiles
  of different parity indicated by white and gray" — the shipped patch
  is a CHT-class window (its center = CHT corner c=(0,1), not the
  rhombus interior: §S5).
- **§7:** "complete algebraic description that parametrizes all
  tilings in the CHT LI class" is OPEN in the paper — out of scope
  here (§STOP 4).

### Pinned construction (implement this; the numbers are the spec)

Work in the module's original lattice coords (u,v); CHT center
**c=(0,1)** (a rhombus corner — the paper's Fig. 6/7 "lower right
corner" hexagon of the shown portion); frame **m0** (mirror frame:
w = (v−1, u)); shipped central state **cstate (a,rbt,sdt)=(0,0,1)**.

Per site x ≠ c, with w = m0(x−c) = (u_w, v_w):

1. **Level/key:** s = min(v2(u_w), v2(v_w)) (v2(0)=∞ sentinel; Rust:
   `i32::trailing_zeros` with the 0-case special-cased), P = w>>s.
2. **Axis:** a = Ta[(P_u&1, P_v&1)] with the pinned 3-entry table
   **Ta: (0,1)→1, (1,0)→2, (1,1)→0**; key (0,0) is unreachable at any
   site (after the min-v2 shift at least one coord is odd) — no
   fallback.
3. **Off-spoke sites** (w on none of the six spoke lines — see 4):
   panel coords (n,(i,j)) from the §5 decomposition
   (`panel_of`: n=1:(u,−v), n=2:(v,u+v), n=3:(−u−v,−u));
   pBS = mod₂⌊i/Q(|i−j|)⌋, pRB = mod₂⌊i/Q(|i+j|)⌋ (SIGNED i, floor
   division for i<0); then
   **rbt = pRB ^ (a==1), sdt = pBS ^ (a==1)** — measured 440/440 exact
   on the rhombus; the a-terms cancel in p = pBS^pRB = Thm 5.
4. **On-spoke sites** (the 12 rays): free bit = Pfold(i)^b,
   companion bit = c (a per-ray constant), where
   Pfold(i) = mod₂⌊i/Q(|2i|)⌋, Pfold(0)=0, Pfold(−i)=1−Pfold(i),
   Pfold(2i)=Pfold(i) (≡ A014577, 64-value cross-check pinned).
   i = signed spoke coordinate (i1→u_w, i2→v_w, i3→−u_w−v_w). The j0
   lines (RB lines, Lemma-8 side) carry FREE rbt; the jp lines
   (stripe lines, Lemma-7 side) carry FREE sdt:

   | ray | line eq. | free | b | c | status |
   |---|---|---|---|---|---|
   | i1j0− | u_w+v_w=0 | rbt | 0 | 1 | observed (0-conf) |
   | i1jp− | u_w−v_w=0 | sdt | 0 | 0 | observed (0-conf) |
   | i2j0− | u_w=0 | rbt | 1 | 1 | observed (0-conf) |
   | i2jp+ | u_w+2v_w=0 | sdt | 1 | 0 | observed (0-conf) |
   | i3j0+ | v_w=0 | rbt | 0 | 0 | observed (0-conf) |
   | i1j0+ | u_w+v_w=0 | rbt | 0 | 1 | pinned (S7 census) |
   | i2jp− | u_w+2v_w=0 | sdt | 1 | 1 | pinned (S7 census) |
   | i3j0− | v_w=0 | rbt | 0 | 1 | pinned (S7 census) |
   | i1jp+ | u_w−v_w=0 | sdt | 0 | 0 | pinned (S7 census) |
   | i2j0+ | u_w=0 | rbt | 1 | 0 | pinned (S7 census) |
   | i3jp− | 2u_w+v_w=0 | sdt | 0 | 1 | pinned (S7 census) |
   | i3jp+ | 2u_w+v_w=0 | sdt | 0 | 0 | pinned (S7 census) |

   The 7 "pinned" rows were selected by census: of the 4⁷=16384
   (b,c)-choices for the 7 unobserved rays, exactly ONE gives
   zero-census R1+R2+R3 on the R=6 hex window (and it survives R=14).
   Winner 12-ray q-pattern (q=b^c, cyclic order
   i1j0+, i2jp−, i3j0−, i1jp+, i2j0+, i3jp−, i1j0−, i2jp+, i3j0+,
   i1jp−, i2j0−, i3jp+): **101011110000**. Secondary open item: 3 of
   the paper's printed S/s patterns (L261) fit the 5 observed rays
   (sA rot=1 comp=0 → 111001110000; sA rot=3 comp=1 →
   100011110001; sB rot=1 comp=0 → 111000110001) but none matches the
   winner exactly — the census is the certification; do not chase the
   S/s correspondence.
5. **Central site** c=(0,1): state = (0,0,1) (the shipped one). The
   other 11 cstates are locally inconsistent in-window (R1/R2 nonzero
   at R=14 — §Battery cstate sweep); they are test-only, never shipped.

**Why no hierarchy walk:** the construction above is a total
O(1)-per-site closed form (Ta total, Lemmas 7/8 total on w≠0, Pfold
total on i≠0, cstate at the center). The paper's §2 level-by-level
hierarchy argument is the *correctness certificate* (constructive
proof that this state assignment is a valid CHT tiling of the whole
plane), not a lookup procedure — no per-site walk needed for
generation; keep the generator cheap (GUI regenerates every frame).

### Pinned evidence (spike trail S1–S7; stdlib Python, ≤240 s each;
scratch at `.scratch/taysoc-plane-recon/`, out files `s5.out`/`s6.out`/
`s7.out`)

- **S1b/S1c (DEAD):** the study's grow3 axis-skeleton law
  (level = min(v2(u),v2(v))+1, axis = (u′+v′) mod 3 at top level)
  self-violates the crossing invariant; best law fit 189/529
  (35.9%). Not the plane path.
- **S5 (the pin, 5.0 s):** 529 candidate centers × 12 frames in 4.3 s.
  UNIQUE optimum: c=(0,1), mirror frames only (m0..m300 all tie;
  rotation frames don't make the cut), pflip=0: **440/440** off-spoke
  sites (100%), online=89. Runners-up: c=(16,−15) 386/453 (85.2%),
  c=(−16,1) 359/453 (79.2%). This explains the study's 57.6% fit
  (304/528): its convention search missed the corner center + mirror
  frame.
- **S6 (0.1 s):** sind violation trends (off-spoke, r=1..6):
  rbt 162/72/24/4/0/0, sdt 199/92/33/9/0/0, a: 0 from r=1 — no finite
  level table closes (127 unobserved keys at r=5); axis-law family
  best 185/528, bit-law family best 249/440 and 241/440 — no closed
  axis/bit law; the §4 frame-coordinate proxy identification produced
  NO fit (the per-ray identification supersedes it).
- **S6b:** raw per-ray dump — on-spoke companion bits are EXACT
  per-ray constants (the pinned ray table's first 5 rows); free bit =
  Pfold(i)^base on every ray, 0-conf.
- **S7 (~15 s):** the off-spoke identification **rbt^pRB == (a==1)
  and sdt^pBS == (a==1): 440/440 exact** (the a-terms cancel in Thm 5);
  Pfold(−k)==A014577(k−1) k=1..64: 64/64; full-state agreement vs the
  shipped 529 table: **529/529**; 4⁷ brute force over unobserved rays:
  **1/16384** zero-census at R=6 (13.0 s), survives R=14 (1/1).
- **Census calibration:** module-verbatim R1/R2/R3 census on the
  shipped 529 table = **R1 0/2992, R2 0/2816, R3 0/2904** (the
  module's battery reproduced).

### Approach (GLB-mapped)

`generate(bbox)` = BBox → lattice window (hex-disk culling in lattice
coords, |du|+|dv|+|du+dv| ≤ 2R; O(window sites)) → per-site state via
the §Pinned construction (O(1)/site: trailing_zeros + div_euclid +
constant tables + Pfold) → the EXISTING 8 convex-piece TEMPLATES
(geometric output unchanged) → emit. No new tile geometry, no new
census. Plane semantics flip the island contract:
`taylor_socolar_empty_beyond_patch` becomes a **beyond-patch
coverage** test (a bbox disjoint from the old rhombus returns
non-empty tiles; `check_coverage`-style sample census on a large view
outside the rhombus passes).

### Battery (all pins programmatic; numbers measured — pin these in-test)

1. **Shipped-table reproduction:** the generator over the 529-site
   rhombus == the shipped SITES table state-for-state: **529/529**
   (cstate (0,0,1)).
2. **Thm-5 parity law:** p = rbt^sdt == pBS^pRB on all 440 off-spoke
   rhombus sites: **440/440**, residue {0}; plus the per-bit
   identities rbt^pRB == (a==1) and sdt^pBS == (a==1): **440/440**.
3. **Pfold cross-check:** Pfold(−k) == A014577 a(k−1), k=1..64:
   **64/64** (a(0..14)=110110011100100; a(4n)=1, a(4n+2)=0,
   a(2n+1)=a(n), a(0)=1).
4. **Local-rule census, module-verbatim R1/R2/R3 on hex windows
   centered at c=(0,1), cstate (0,0,1)** (window site counts
   3R²+3R+1 = 127/217/331/631; the windows extend beyond the shipped
   rhombus in all directions — its far corners, e.g. (22,−21), sit at
   hex-radius ~22, outside R=14):
   - R=6: R1 0/684, R2 0/612, R3 0/648
   - R=8: R1 0/1200, R2 0/1104, R3 0/1152
   - R=10: R1 0/1860, R2 0/1740, R3 0/1800
   - R=14: R1 0/3612, R2 0/3444, R3 0/3528
5. **Cstate sweep (R=14, winner config):** R3 = 0/3528 for ALL 12
   cstates (Thm 3 — no single-color vertex); only cstate (0,0,1) has
   R1=R2=0. Others (R1/R2 of 3612/3444): (0,0,0): 4/0; (0,1,0): 4/4;
   (0,1,1): 0/4; (1,·,·) and (2,·,·): 8/8. (Test-only, not shipped.)
6. **8-piece tile-area identity** (module's existing in-module test):
   the 8 convex pieces per hexagon still exactly partition the hexagon;
   unchanged by this brief.
7. **Baseline battery intact:** the shipped-table R1/R2/R3 battery
   (0/2992, 0/2816, 0/2904) keeps passing on the shipped table;
   extend the battery with the hex-window censuses of item 4 as
   in-module tests.
8. **Plane coverage (THE new headline):** `check_coverage`-style
   sample census on the standard trio bboxes AND a large view well
   outside the old rhombus (e.g. ±20 world units at unit emission):
   every sampled point covered exactly once, clipped-area equality to
   1e-6 rel.; the flipped beyond-patch test of the Approach section.
9. **Determinism** (geometry, proto, seed) — unchanged requirement.

### Census deltas: NONE

Registry id `taylor-socolar` unchanged; aperiodic family count
unchanged; built-ins unchanged; memberships unchanged. **If either
orchestrator census guard fires, that is UNEXPECTED — STOP and report
verbatim.** Owned surface notes: `docs/families/aperiodic.md` row 20
(presentation note: "plane-filling CHT closed-form presentation; M86
shipped the forced 529-hexagon patch") + rows ~247–248 (the 57.6%
inflation-fit admission is now RESOLVED: the study's convention
search missed the corner center c=(0,1) + mirror frame; with the
pinned construction the off-spoke closed form is exact 440/440 and the
full state reproduces the patch 529/529) + the matching module-header
doc lines (L~110–118 honesty block).

### Files you own

- `crates/tess-tilings/src/families/aperiodic/taylor_socolar.rs` —
  the plane presentation upgrade (generate() per §Approach; the pinned
  construction; flipped beyond-patch test; extended battery; header
  doc update). Keep the 8-piece TEMPLATES and the certified core
  intact.
- `docs/families/aperiodic.md` — row 20 note + the 57.6% admission
  update (owned row; no new build arc needed — row-only).
- Scratch `/tmp/opencode/taylor-socolar-plane/` (Python spike,
  stdlib only; the pinned trail at `.scratch/taysoc-plane-recon/`
  is READ-ONLY reference).

**Do not touch:** `lib.rs` (census pin, guards), `README.md`,
`docs/INDEX.md`, `docs/history/CHANGELOG.md` (M number + entry are the
orchestrator's), other families, UI crates, or the citation line
(unverified — flag only). One worktree commit:
`aperiodic-family: taylor-socolar plane-filling presentation (CHT closed form)`.
Constraints: `cargo check --workspace` zero warnings; NO repo-wide
`cargo fmt`; exact integer arithmetic only (Q = 1 << v2; floor
division for negative i via `div_euclid` — truncating division is
WRONG for i<0); MAX_TILES = 250_000 with a cheap O(window) generator;
floats at emission only.

### STOP conditions

- **Pin mismatch:** if the Rust generator fails to reproduce ANY of
  the pinned numbers — 529/529 table agreement, 440/440 Thm-5 /
  per-bit identities, 64/64 Pfold cross-check, or any zero-census
  total of item 4 (0/684, 0/612, 0/648, 0/1200, 0/1104, 0/1152,
  0/1860, 0/1740, 0/1800, 0/3612, 0/3444, 0/3528) — STOP and report
  the exact mismatch. The pinned construction is the spec; do not
  "fix" it silently.
- **Coverage failure:** if any plane-coverage census (item 8) shows a
  gap or overlap anywhere in the tested views — STOP; a plane patch
  that cannot be proven gap-free/overlap-free must NOT ship; report
  the failing census verbatim.
- **Census move / guard fire:** this brief has zero census deltas —
  STOP (the guards are the backstop against a wrong construction).
- **Scope guard:** §7's complete closed form parametrizing ALL CHT LI
  class members is OUT of SCOPE (open in the paper). If the pinned
  single member proves insufficient and a general CHT generator is
  needed, STOP and open a new brief — do not start it here.
Never guess; measure and report.
