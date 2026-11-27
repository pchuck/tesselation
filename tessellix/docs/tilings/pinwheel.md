# Pinwheel (1-2-√5)

`pinwheel` · family [Arrangements](../families/arrangements.md) (`arrangements`)

non-periodic Pythagorean; tiles in infinitely many orientations

## Algorithm

**Pinwheel** (`pinwheel.rs`) — Conway–Radin tiling by the 1–2–√5
  right triangle; √5 inflation with an `arctan(1/2)` twist, so tiles
  occur in infinitely many orientations. Labeled-vertex child placement
  handles both chiralities. The substitution is not edge-to-edge (four
  children meet at the midpoint of a long leg); those T-junctions are
  inherent and verified by tests, not bugs. The emitted patch is
  view-independent: seed grids are anchored to the origin and quantized
  to √5 octaves, so panning/zooming only re-culls a fixed global tiling
  (the seed depth steps once when the view extent crosses a √5 octave),
  and "pattern" mode colors tiles by their substitution slot — the five
  distinct child roles of the rule.

## See also

- Family doc (catalog table + build arcs): [docs/families/arrangements.md](../families/arrangements.md)
- Mathematical background: [docs/2D_Tilings_Reference.md](../2D_Tilings_Reference.md)
- Render it: `cargo run -p tess-tui -- export --tiling pinwheel --mode proto --out pinwheel.svg`

---

*Per-tiling document of the [tessellix](../../README.md) registry · [documentation index](../INDEX.md)*
