## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output
- **Max 2 `fast-worker` tasks in flight at a time** (owner directive 2026-11-12); queue additional fast-worker briefs until a slot frees
- Prefer 'worker' for most development iterations. Can launch 2 in parallel.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use 'frontier' for the most challenging tasks, or when other workers aren't converging on a solution.
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
