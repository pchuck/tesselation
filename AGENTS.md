## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output.
- Prefer `worker` for most development iterations.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use `frontier` for the most challenging tasks, or when other workers aren't converging on a solution.
- **One job per agent at a time.** Each agent of every tier (`worker`, `strong-worker`, `fast-worker`, `frontier`) is dispatched exactly one job at a time — never give a busy agent a second job, never multiplex tasks within one session. Agents may run concurrently, one job each: each agent's session executes on its own system, so agent concurrency is not limited by the coordinator's local box. Launch one job at a time and confirm it is established, then decide on the next dispatch; never batch multiple launches.
- Briefs must be self-contained: goal, files, constraints, how to verify.

## Harness discipline
- **Launch every subagent with `background: true`.** Foreground launches block the whole session; the launch receipt's "working in the background" line is boilerplate, not the mode — the parameter is what counts.
- **Never end a turn while a background job runs** — poll synchronously with bounded sleeps. Coordinators keep this rule in every worker brief.
- **Vision policy (two-tier):** model vision is allowed for reconnaissance — comparing a render against a reference figure, catching gross structural errors early, UI appearance judgment — but never as verification evidence: every count, closure, pairing, or correctness claim in a report or test must be programmatic (parsed SVG/path data, exact arithmetic, census battery, mutation-checked headless test). When a visual look motivated a change, the report must cite the programmatic check that confirmed it. "Looks right" is not a deliverable.