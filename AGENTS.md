## Delegation rules
- Prefer `fast-worker` for discrete small tasks with clear tests or well-defined output.
- Prefer `worker` for most development iterations.
- If a worker fails twice, summarize the failure and call `strong-worker`.
- If `strong-worker` doesn't resolve it after a few tries, call `frontier` with the failing output and what was tried.
- Use `frontier` for the most challenging tasks, or when other workers aren't converging on a solution.
- **One job per agent at a time.** Each agent session is dispatched exactly one job at a time — never give a busy agent a second job, never multiplex tasks within one session. Launch one job at a time and confirm it is established, then decide on the next dispatch; never batch multiple launches.
- **Tier parallelism:** all worker tiers run on separate hardware and can handle tasks in parallel. `worker` and `fast-worker` can each handle up to two jobs in parallel; `strong-worker` and `frontier` are sized to each handle one large/complex task at a time.
- **Keep idle tiers fed.** When workers are idle and dependency-free work is in the queue, feed open tasks of appropriate difficulty to the idle worker(s) — match the task's difficulty to the tier (fast-worker: discrete/mechanical; worker: standard; strong-worker: hard geometry/correctness; frontier: novel derivation). Work whose inputs depend on an in-flight job stays queued until that job lands.
- Briefs must be self-contained: goal, files, constraints, how to verify.

## Harness discipline
- **Persistent scratch:** `/workspace/tmp` is the shared persistent scratch location — it survives reboots and infrastructure restarts. `/tmp/opencode` is fast ephemeral scratch and can be wiped at any time. Write batteries, pinned derivations, and salvage files directly to `/workspace/tmp` (or checkpoint them there from `/tmp/opencode` before reporting). Never treat `/tmp/opencode` as the only copy of hard-won work.
- **Launch every subagent with `background: true`.** Foreground launches block the whole session; the launch receipt's "working in the background" line is boilerplate, not the mode — the parameter is what counts.
- **Background jobs notify on completion** — after launching background subagents, end the turn; the harness wakes the session as each job finishes, and integration proceeds then. Do not poll the session DB with sleep loops.
- **Vision policy (two-tier):** model vision is allowed for reconnaissance — comparing a render against a reference figure, catching gross structural errors early, UI appearance judgment — but never as verification evidence: every count, closure, pairing, or correctness claim in a report or test must be programmatic (parsed SVG/path data, exact arithmetic, census battery, mutation-checked headless test). When a visual look motivated a change, the report must cite the programmatic check that confirmed it. "Looks right" is not a deliverable.