## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output.
- Prefer `worker` for most development iterations.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use `frontier` for the most challenging tasks, or when other workers aren't converging on a solution.
- **One job at a time — strictly.** The coordinator never has more than ONE job in flight across all tiers (`worker`, `strong-worker`, `fast-worker`, `frontier`). Dispatch a job, wait for it to complete (or be stood down), then dispatch the next. No parallel dispatch, no batched launches, no running queue. Every other task waits in the queue until the in-flight count is zero. Tier choice and the escalation ladder still apply per job.
- Briefs must be self-contained: goal, files, constraints, how to verify.

## Harness discipline
- **Launch every subagent with `background: true`.** Foreground launches block the whole session; the launch receipt's "working in the background" line is boilerplate, not the mode — the parameter is what counts.
- **Never end a turn while a background job runs** — poll synchronously with bounded sleeps. Coordinators keep this rule in every worker brief.
- **Vision policy (two-tier):** model vision is allowed for reconnaissance — comparing a render against a reference figure, catching gross structural errors early, UI appearance judgment — but never as verification evidence: every count, closure, pairing, or correctness claim in a report or test must be programmatic (parsed SVG/path data, exact arithmetic, census battery, mutation-checked headless test). When a visual look motivated a change, the report must cite the programmatic check that confirmed it. "Looks right" is not a deliverable.