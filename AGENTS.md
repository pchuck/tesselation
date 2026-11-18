## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output
- Prefer `worker` for most development iterations.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use 'frontier' for the most challenging tasks, or when other workers aren't converging on a solution.
- **Parallel residency allowed; dispatch one job at a time (owner directive
  2026-11-18; supersedes the 2026-11-12 "max 2 `fast-worker` in flight" cap
  and the "launch 2 workers in parallel" rule).** Agents of every tier
  (`worker`, `strong-worker`, `fast-worker`, `frontier`) may run in
  parallel, but the coordinator dispatches new jobs ONE AT A TIME: launch
  a single job, confirm it is established and the box is healthy, then
  decide on the next dispatch. Never batch multiple launches. Context:
  this box's 15.5 GiB GPU twice OOM-killed a strong-worker session while
  another session was generating (2026-11-17/18); heavy derivations are
  the marginal resident — when one is in flight, observe between
  dispatches and prefer queueing over adding a second generating session.
- Briefs must be self-contained: goal, files, constraints, how to verify.

## Harness discipline
- **Launch every subagent with `background: true`.** Foreground launches
  block the whole session and force the owner to detach manually (^b).
  The launch receipt's "working in the background" line is boilerplate,
  not the mode — the parameter is what counts. (2026-11-17: four
  consecutive launches needed manual detaching before this was fixed.)
- **Never end a turn while a background job runs** — poll synchronously
  with bounded sleeps (origin: M64 att-3 session cut). Coordinators keep
  this rule in every worker brief.
- **Vision policy (two-tier; supersedes the blanket no-model-vision ban,
  2026-11-17):** model vision is allowed for reconnaissance — comparing
  a render against a reference figure, catching gross structural errors
  early, UI appearance judgment — but never as verification evidence:
  every count, closure, pairing, or correctness claim in a REPORT or a
  test must be programmatic (parsed SVG/path data, exact arithmetic,
  census battery, mutation-checked headless test). When a visual look
  motivated a change, the report must cite the programmatic check that
  confirmed it. "Looks right" is not a deliverable.
