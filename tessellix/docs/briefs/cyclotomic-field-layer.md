# Brief: cyclotomic-field-layer — design doc + exact-arithmetic spike

**Family:** none (cross-cutting research; no registry changes) ·
**Candidate:** PROPOSED.md §23e (GLB binary re-scope: "needs a
cubic/quartic cyclotomic field layer we do not ship yet — frame as
its own milestone with a field-layer design doc first") and §23i
(Danzer 7-fold, Harriss 9-fold, Goodman-Strauss 7-fold, CAST 10-fold,
Nischke–Danzer deltoids, Say-awen 17/21-fold — "each needs degree-3/4
arithmetic before any substitution table; effort XL per family
(design docs first; overlaps #18)") · **Effort:** L · **Tier:**
frontier (novel derivation).

### Goal

Produce the **field-layer design doc** that §23e/§23i demand as the
first step: an exact-arithmetic plan for degree-3 and degree-4
cyclotomic real subfields — K₇ = Q(2cos(π/7)) (degree 3), K₉ =
Q(2cos(π/9)) (degree 3), K₁₄ = K₇, K₁₈ = K₉, and the quartic cases
K₁₅ = Q(2cos(π/15)), K₁₆ = Q(2cos(π/16)), K₂₀/₂₁ similar — sufficient
for a reviewer to implement the layer in `tess-core`/`tess-tilings`
without re-deriving anything. Deliverable is a **document + a
stdlib-only Python proof-of-concept**, not registry code.

### Specifics the doc must pin

1. **Element representation** per field: the integer coefficient
   tuple (c₀ + c₁a + … , a = 2cos(π/n)) with exact reduction tables
   (minpoly aᵈ − … via the Chebyshev identities), following the
   shipped quartic precedent `girih_qc.rs` kmul (a⁴ = 5a² − 5) and
   the salvaged `gexact.py` tuple model (archive, see below).
2. **Exact constants with certificates** per field: cos/sin of the
   frame angles (36°-style frames generalize to 2π/n multiples),
   derived by algebraic identities, each with an exact proof and a
   float cross-check.
3. **Rotation matrices on the n-fold frame**: exact rotation of
   vectors by the frame's unit directions U(k) — the girih-primitive
   spike's `U(k)` edge-vector frame (36° multiples) is the shipped
   pattern; generalize to 2π/n.
4. **Sign/ordering machinery**: exact comparison (the Z[√3]/Z[√φ]
   zcmp pattern) needed for canonical dedup and PIP checks.
5. **Per-family applicability table**: which §23i families need which
   field (Danzer 7-fold → K₇, Harriss 9-fold → K₉, CAST 10-fold →
   Q(√5) — already shipped? verify, Say-awen → K₂₁, GLB binary n=7/9
   → K₇/K₉, n=15 → K₁₅ degree-4…), with the substitution-table
   effort estimate per family once the layer exists.
6. **Implementation plan**: where the layer lives (tess-core?
   tess-tilings common module?), trait shape (a `Field`-like
   abstraction vs per-field concrete types — recommend one, argue
   from the shipped surd/quartic precedents), and the migration path
   for existing fields (keep them; new fields opt-in).

### Salvage + literature

- Salvage archive (read-only, local):
  `/home/pcharles/dev/pchuck/tesselation/.scratch/salvage-20261120/`
  — `tmp/girih_prim/gexact.py` is the shipped quartic field's tuple
  model ("field rule verbatim from girih_qc.rs kmul", exact
  certificates for √5/φ/cos36/sin36, and a recorded warning that
  float math in the old environment was unreliable — exact algebra
  first, floats only as cross-checks).
- Literature: standard cyclotomic-field facts (real subfields of
  Q(ζₙ), degrees φ(n)/2, Chebyshev minpolys); Conway–Guy–or standard
  references for constructible-polygon context. Cite what you use.
- BANNED: never view/dump the salvage PDFs (`paper.pdf`, `som.pdf`).

### Deliverables (worker report + files)

1. The design doc as a markdown file at
   `/tmp/opencode/cyclotomic-design/design.md` (the orchestrator
   integrates it into `docs/` — do not write into the repo's docs/
   yourself). Target 200–400 lines; every claim either proven in-doc
   (exact algebra) or cited.
2. The proof-of-concept spike: stdlib-only Python in the same scratch
   dir — exact multiplication/reduction + the U(k) rotation frame +
   one worked example per field (K₇ and K₉ minimum, K₁₅ if time
   allows), cross-checked against float numerics. Numbers in the
   report.
3. The report: doc outline, the per-family applicability table, spike
   numbers, open questions (e.g. K₂₁ tower costs, whether CAST
   10-fold truly needs only Q(√5)).

### Constraints

- **No repo changes at all** (no worktree edits, no commits) — this
  is a research brief; everything lives in scratch + the report.
- Python stdlib only; ≤240 s per experiment.
- Exact algebra is the evidence; floats only cross-check.
- Worktree (read-only reference): create none — read the repo at
  `/home/pcharles/dev/pchuck/tesselation/tessellix/` directly
  (read-only); do not modify anything there.
- Keep NOTES.md in the scratch dir so state survives a session death.