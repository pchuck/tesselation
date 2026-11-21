# Brief: fields-layer-k7 — exact cyclotomic field core (K₇) in Rust

**Family:** none — cross-cutting core infrastructure (run SOLO per
`AGENTS.md`; no family work touches these files). **Spec:**
[docs/cyclotomic-field-layer.md](../cyclotomic-field-layer.md) — the
delivered design doc; §7 is the implementation plan, §2/§4/§5 the
mathematics. **Scope:** §7 step (1) only — `fields.rs` + **K₇** + its
test battery. K₉/K₁₅/L₁₀ and the girih ROTM anchor test are follow-up
landings (step 2); **no consumer family yet** (step 3 is a separate
brief). **Effort:** L · **Tier:** frontier.

### Sanctioned exception (this brief only)

`crates/tess-tilings/src/lib.rs` is orchestrator-owned, but a new
top-level module needs its declaration. You may add **exactly one
line** to lib.rs: `mod fields;` (plus nothing else — no test changes,
no census, no guards). Everything else lives in files you own.

### Files you own

- `crates/tess-tilings/src/fields.rs` (NEW — the whole deliverable).
- `crates/tess-tilings/src/lib.rs` — ONLY the `mod fields;` line.
- `/tmp/opencode/fields-layer-k7/` — scratch; append-only NOTES.md.

**Do not touch:** anything else in lib.rs, any families/ file,
README, docs/ (the design doc already records the plan), UI crates,
tess-core, AGENTS.md, Cargo.toml (see deps rule).

### Deliverable (mirroring doc §7 exactly)

```rust
pub struct K7(pub [i64; 3]);   // c0 + c1·b + c2·b², b = 2cos(2π/7),
                               // minpoly m(x) = x³ + x² − 2x − 1
```

- Reduction/mul from minpoly consts (convolution to degree 4, fold
  with `b³ = 1 + 2b − b²`, `b⁴ = −1 − b + 3b²`) — derived constants
  pasted from the doc §2.1 table (they are Sturm-certified there;
  do not re-derive in Rust).
- Generic helpers as free functions over `(deg, minpoly, iso)` where
  practical, so K₉/K₁₅ land as ~40-line additions later (doc §7).
- Companion rotation `R = [[0,−1],[1,b]]` and the oblique frame:
  `P7(K7, K7)` point type with dot (`A·A' + B·B' + (A·B' + A'·B)·b/2`
  → keep exact: work in tuples, result K7), norm² (`A² + B² + A·B·b`),
  cross-sign via `sign(A·B' − A'·B)` (doc §4.1 — the sine-free trick),
  `rot` by R^k.
- Exact sign/order engine `ksign` (doc §5): exact zero test (all-zero
  tuple), else interval-Horner over the Sturm isolating interval of b
  (rightmost real root — paste the interval from the spike battery),
  bisect on undecided; ordering = sign(x − y). **No new crate
  dependencies** (tess-core is zero-dep; hand-roll the rational
  interval arithmetic as (i64 num, i64 den) pairs).
- f64 appears only in cross-check tests, never in the exact path.

### Test battery (all in `fields.rs` `#[cfg(test)]`; mirrors doc §7 +
spike battery — port the spike's element lists from
`.scratch/cyclotomic-design/spike.py` / `spike_output.txt`)

1. `m(b) == 0` on the tuple model (exact); float cross-check ≤ 1e−12.
2. Frame: `|U(k)|² == 1`, `Σ_k U(k) == 0`, `R^k U(0) == U(k)`,
   `R^k U(1) == U(k+1)` for all k (exact K7 equality).
3. Star walks `{7/1}`, `{7/3}`: unit-edge `±U(k)` steps close exactly;
   centered walk `R^s`-equivariant (`R^s E_j = E_{j+1}`); exact
   shoelace factor in K7 ({7/1}: `4 + 3b + b²`), float area agreement.
4. Sign/order: the spike's 29-element battery — exact signs match
   float signs; exact total order matches float sort (distinct floats
   asserted).
5. Norm identities (test-only is fine): `N(1) = 1`, `N(b) = (−1)³·m₀
   = 1`, multiplicativity (Sylvester, exact i128).

### Constraints

- `cargo check --workspace` zero warnings; `cargo fmt` your hunks;
  `cargo test --workspace` fully green (this landing changes NO census
  surface — fields.rs registers no tilings — so every guard must pass
  untouched).
- i64 core (d = 3, D = 1); overflow notes in doc §2 Tier E.
- No M number, no pushes: ONE worktree commit,
  subject `fields-layer: K7 exact field core (companion frame + Sturm sign)`.

### Report back

Diff scope; the test list with exact pass counts; `cargo check`/`test`
totals; any deviation from the doc (and why); confirm lib.rs diff is
exactly one line.