## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output
- **Max 2 `fast-worker` tasks in flight at a time** (owner directive 2026-11-12); queue additional fast-worker briefs until a slot frees
- Prefer 'worker' for most development iterations. Can launch 2 in parallel.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use 'frontier' for the most challenging tasks, or when other workers aren't converging on a solution.
- Briefs must be self-contained: goal, files, constraints, how to verify.
