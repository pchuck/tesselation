# Jeandel-Rao 11-tile Wang tiles

`jeandel-rao` · family [Aperiodic](../families/aperiodic.md) (`aperiodic`)

aperiodic 11-tile edge-colored Wang set (Jeandel-Rao, arXiv:1506.06492); the author's canonical 100x100 patch, each unit cell partitioned into 5 sub-polygons whose edge bands carry the Wang edge colors (H 0/9900, V 0/9900, exact 9-color ids, re-derived in tests); finite covered patch, no period |t|<=30

## Algorithm

**Jeandel-Rao 11-tile Wang tiles** (`jr_aperiodic.rs`) — the smallest
  known aperiodic set of Wang tiles (edge-colored unit squares, no
  rotations or reflections; Jeandel-Rao, arXiv:1506.06492), ported as a
  data table from the audited upstream study (M93): the author's
  canonical 100x100 patch, every unit cell partitioned into 5
  non-overlapping convex sub-polygons — four edge bands of width 0.08
  plus the interior remainder — whose band colors make the Wang edge
  matching visible exactly where the handoff colors agree (H 0/9900,
  V 0/9900 on exact 9-color ids, re-derived in tests); tile frequencies
  and a radius-30 no-period probe are pinned against the study.
  Aperiodicity is cited (machine-verified upstream), not re-proved here;
  the covered domain is the finite patch (Taylor-Socolar precedent), not
  the plane.

## See also

- Family doc (catalog table + build arcs): [docs/families/aperiodic.md](../families/aperiodic.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling jeandel-rao --mode proto --out jeandel-rao.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
