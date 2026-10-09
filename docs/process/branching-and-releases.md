# Branching and releases

The rules an agent must follow are summarised inside AGENTS.md. This file has the detail, the one-time setup and the release checklist. If the two ever disagree, stop and ask.

## Model

```
feat/s01-unit-service  --\
doc/update-adr-0010    ---+--> PR (squash) --> develop --> PR (merge commit) --> main
fix/offset-corner-bug  --/    staging, testing,           production,
                              tags vX.Y.Z-rc.N            tags vX.Y.Z
```

- main is production. Only tested, tagged releases arrive there. You merge to it.
- develop is staging and testing. Finished work accumulates here, is tested together, and release candidates are tagged here.
- feat, doc and fix branches are short-lived and hold one piece of work each. Work accumulates on develop through pull requests, then moves to main in one pull request.

## Branches

| Branch | Starts from | Merges into | Method | Lifetime |
|---|---|---|---|---|
| feat/<id>-<name> | develop | develop | Squash merge | Until its PR is merged, one to two days |
| doc/<topic> | develop | develop | Squash merge | Short |
| fix/<topic> | develop | develop | Squash merge | Short |
| develop | main (once, at setup) | main | Merge commit | Permanent |
| main | n/a | n/a | n/a | Permanent |
| fix/hotfix-<topic> | main | main, then back into develop | Merge commit | Only for urgent production defects |

Why these merge methods: a squash merge gives develop one clear commit per slice. The develop to main merge uses a merge commit, never a squash, so the two long-lived branches keep a shared history and do not drift apart.

## Names and commits

- Prefixes: feat/, doc/, fix/ only. Lowercase, hyphens, slice ID first: feat/s01-unit-service, feat/sp-02-offset, doc/review-after-s02, fix/dart-label-after-recompute.
- Spikes use feat/sp-nn-<name>. Their PR is closed, not merged. The finding goes in an ADR on a doc/ branch.
- Commit messages: `<type>(<scope>): <summary>`. Types: feat, fix, doc, test, refactor, chore. Scope is the slice ID or area. The body lists requirement IDs and test IDs.
- Pull request title: `S01: unit service`. The body is the completion report (see .github/pull_request_template.md).

## Flows

### Start a slice

```bash
git fetch origin
git switch -c feat/s01-unit-service origin/develop
```

### Finish a slice

```bash
git fetch origin && git rebase origin/develop
# run every verification command from AGENTS.md
git push -u origin feat/s01-unit-service
# open a PR into develop and fill in the template
```

You review the diff and the evidence, wait for CI, and squash merge. Delete the branch after merging.

### Release (develop to main)

1. develop is green on its latest commit.
2. Tag the release candidate on develop: `git tag v0.1.0-rc.1 && git push origin v0.1.0-rc.1`. CI builds it.
3. Test the candidate: all verification commands, the manual procedures that apply to the included slices (docs/testing/manual-procedures.md), and the checks in the list below.
4. Fix problems with fix/ branches into develop, then tag rc.2 and repeat.
5. When the candidate passes, set Verified in traceability.csv for the requirements that passed their human gates, and update the review log.
6. Open a pull request from develop to main. Use a merge commit. Merge it yourself.
7. Tag main: `git tag v0.1.0` on the merge commit and push it.

Release checklist:

- CI is green on develop and on the release candidate tag.
- Every included slice has a completion report in its merged PR.
- Open questions that block any included slice are decided.
- Manual procedures required by the included slices are recorded under docs/testing/evidence/.
- No requirement is marked Verified without the evidence in docs/testing/evidence-and-dod.md.
- The WASM size gate passes once S16 exists.
- docs/requirements and docs/architecture match what shipped.

### Hotfix (rare)

Only the lead developer starts one. Branch fix/hotfix-<topic> from main, fix with a failing test first, open a PR into main, merge, tag a patch version, then merge main back into develop.

### If develop is red

Stop feature work. Open a fix/ branch from develop to restore green. Merge it before any other PR.

## Tags and versions (proposal)

Semantic versions. Stay at 0.x while R1 is under construction. Use v1.0.0 for the first release whose R1 evidence pack is Verified (slice S18). Release candidates are v0.2.0-rc.1 style. Only the lead developer creates tags.

## Who does what

| Action | Agent | Lead developer |
|---|---|---|
| Create feat, doc, fix branch from develop | Yes | Yes |
| Commit and push to its own branch | Yes | Yes |
| Open PR into develop | Yes | Yes |
| Merge PR into develop | No | Yes |
| Set status Implemented | Yes | Yes |
| Set status Verified | No | Yes |
| Tag rc or release | No | Yes |
| Open or merge develop to main PR | No (may draft the description if asked) | Yes |
| Hotfix on main | Only when told, and still no merge | Yes |

## One-time repository setup

1. Make the first commit on main with the docs, AGENTS.md and this folder. This is the only direct commit to main.
2. Create develop: `git switch -c develop && git push -u origin develop`.
3. On the host, make develop the default branch, so new pull requests target it by default. Agents still must never target main.
4. Turn on branch protection or rulesets for main and develop where your plan allows: require a pull request, require CI checks once S00 creates them, block force pushes and deletion, and allow merging only by you. If your plan does not offer these for your repository type, the rules in AGENTS.md and your own discipline are the control.
5. Allow squash merging and merge commits. Turn on automatic deletion of merged branches.
6. CI must run on pull requests into develop and main, on pushes to develop and main, and on tags beginning with v (S00 acceptance).

## Open items

- Whether you want the agent to ever merge into develop (currently no).
- Version numbering: the proposal above.
- The CI host is still OQ-34. The pull request template assumes GitHub.
