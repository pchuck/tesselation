# Uniform (Archimedean) family

The eleven uniform (Archimedean) tilings — the three regular patterns plus the eight semiregular ones — as exact `LatticeTiling` data tables. The set is **closed**: no further Archimedean tilings exist.

Code: `crates/tess-tilings/src/families/uniform/` — cargo feature `uniform` (default-on).
Adding a tiling here touches only this family: `families/uniform.rs`
(module decl, register slice, classification arms, census `IDS`,
`lattice_tilings` entries) plus the new module and its doc row below.
Work briefs: `docs/briefs/TEMPLATE.md`; fleet protocol: `tessellix/AGENTS.md`.

## Implemented (11 tilings)

| id | vertex figure | group | notes |
|---|---|---|---|
| `square` | 4.4.4.4 | p4m | regular; self-dual |
| `hexagon` | 6.6.6 | p6m | regular |
| `triangle` | 3.3.3.3.3.3 | p6m | regular |
| `truncated-square` | 4.8.8 | p4m | octagons + squares |
| `trihexagonal` | 3.6.3.6 | p6m | kagome |
| `snub-trihexagonal` | 3.3.3.3.6 | p6 | chiral |
| `elongated-triangular` | 3.3.3.4.4 | cmm | non-Wythoffian |
| `rhombitrihexagonal` | 3.4.6.4 | p6m | |
| `truncated-trihexagonal` | 4.6.12 | p6m | great rhombitrihexagonal |
| `truncated-hexagonal` | 3.12.12 | p6m | |
| `snub-square` | 3.3.4.3.4 | p4g | chiral-looking, mirror-symmetric |

The uniform family is **closed**: all 11 exist, so no further
Archimedean work is possible (or needed).

## Build arcs

Numbered arcs keep their PATTERNS.MD §numbers — code comments and
briefs cite them as `§N`.

Historical note: the eleven Archimedean tilings shipped in the
foundation era (M1–M18) as part of the initial "complete the classics"
arc — they predate the milestone changelog (the first CHANGELOG entry is
the M19–M27 arc) and carry no per-tiling `§N` arcs, so no numbered
build arcs are written here.

## Per-tiling documents

Each member's full specification — algorithm, exact geometry, verification, sources — lives in its own document under [docs/tilings/](../tilings/):

- [`square`](../tilings/square.md) — Square grid
- [`hexagon`](../tilings/hexagon.md) — Hexagonal
- [`triangle`](../tilings/triangle.md) — Triangular
- [`truncated-square`](../tilings/truncated-square.md) — Truncated Square (4.8.8)
- [`trihexagonal`](../tilings/trihexagonal.md) — Trihexagonal (3.6.3.6)
- [`snub-trihexagonal`](../tilings/snub-trihexagonal.md) — Snub Trihexagonal (3.3.3.3.6)
- [`elongated-triangular`](../tilings/elongated-triangular.md) — Elongated Triangular (3.3.3.4.4)
- [`rhombitrihexagonal`](../tilings/rhombitrihexagonal.md) — Rhombitrihexagonal (3.4.6.4)
- [`truncated-trihexagonal`](../tilings/truncated-trihexagonal.md) — Truncated Trihexagonal (4.6.12)
- [`truncated-hexagonal`](../tilings/truncated-hexagonal.md) — Truncated Hexagonal (3.12.12)
- [`snub-square`](../tilings/snub-square.md) — Snub Square (3.3.4.3.4)

---

*Map: [docs/INDEX.md](../INDEX.md) · Changelog: [docs/history/CHANGELOG.md](../history/CHANGELOG.md) · Queue: [docs/ROADMAP.md](../ROADMAP.md)*
