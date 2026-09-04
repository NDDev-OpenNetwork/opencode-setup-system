# Writing this harness's instruction file

Generated from `references/opencode-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

## Where it goes

`~/.config/opencode/AGENTS.md`

Decided by: https://opencode.ai/docs/rules

## What the record says about it

second in the search order, after a project's own

**Searched in the product's own pinned bytes on 2026-08-29 and not found, which argues nothing either way.** Fixed-string, anchored to this product's configuration home -- the bare leaf name is in every one of these binaries and proves nothing, so only the anchored form counts. An invented path was searched in the same run and was also absent, so the search discriminates.

This row stays `page` because **a path built by joining a directory to a name at runtime never appears as a literal**, and that is the shape of every remaining one. Moving it off `page` needs the product run against a target and asked what it resolved, not a deeper grep.

**Off `page` on 2026-08-31, and the near-miss is the part worth keeping.** The 1.18.25 binary resolves its global instruction set as `[join(Global.config, "AGENTS.md"), join(Global.home, ".claude", "CLAUDE.md")]`, takes the **first** of those that exists and stops. The question was what `Global.config` is, and `debug paths` answers it wrongly: that command iterates the *static* path table, which is XDG-derived and prints `~/.config/opencode` even when `OPENCODE_CONFIG_DIR` is set. The `@opencode/Global` **service** is a different object and returns `config: OPENCODE_CONFIG_DIR ?? <static>`, so a target's `AGENTS.md` is the one read.

Stopping at the command's output would have concluded this file is inert under provider launch -- which would have made `minimal`, whose whole content is this file, a posture that installs nothing. It is not. The reading was plausible, the command was real, and the object it prints is not the object the resolver uses.

**And the second entry is another product's file.** With no `AGENTS.md` at the configuration home, this product reads another harness's global instruction file, `CLAUDE.md` under that product's own home in `$HOME`, as its own, unless `OPENCODE_DISABLE_CLAUDE_CODE_PROMPT` is set. Every posture here ships `AGENTS.md`, and it is first in the list, so ours wins wherever a setup is installed -- the fallback matters for a target this provider has not touched.

## Where the other harnesses keep theirs

| harness | path | shape |
|---|---|---|
| `antigravity` | `config/rules` | directory |
| `claude` | `CLAUDE.md` | file |
| `codex` | `AGENTS.md` | file |
| `cursor` | `rules` | directory |
| `grok` | `AGENTS.md` | file |
| **this one** | `AGENTS.md` | file |
| `pi` | `AGENTS.md` | file |

**They are not interchangeable, and the difference is not only the
name.** One of the seven takes a *directory* of rules rather than a
single document, so a file moved between the two is not a rename.

**Some products read a neighbour's.** `references/surfaces.md` records
every such cross-read this estate has measured, on the declined rows:
a file written for one product can change what a second one sees, and
removing a setup can change what a third one sees. That is a property
of the products, not of this program, and it is the reason the declined
list is worth reading before writing here.

## Before you write one

- **This file is the floor, not the ceiling.** A repository's own
  instructions sit above it; write what is true everywhere and leave
  the rest to the project.
- **Read it back where the product reads it**, not where the install
  put it. Several of these products resolve a home through an override
  chain, and the two are not always the same directory.

