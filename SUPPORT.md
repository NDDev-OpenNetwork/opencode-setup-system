# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is declared. It starts the exact executable a software install
placed under `--prefix`, never a name found on `PATH`, and points the
product at `--target` through the environment variable its own
documentation names.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.config/opencode`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `AGENTS.md` | `instruction` | [source](https://opencode.ai/docs/rules) |
| `opencode.json` | `setting` | [source](https://opencode.ai/docs/config) |
| `skills` | `skill` | [source](https://opencode.ai/docs/skills) |
| `agents` | `agent` | [source](https://opencode.ai/docs/agents) |
| `commands` | `command` | [source](https://opencode.ai/docs/commands) |
| `plugins` | `plugin` | [source](https://opencode.ai/docs/plugins) |
| `tui.json` | -- | [source](https://opencode.ai/docs/tui) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`opencode.jsonc`** -- Documented, and deliberately not owned. OpenCode reads either spelling; owning both would let a target hold two documents that disagree, with the product picking one and this provider reporting the other. Owning one keeps the answer single, and a target configured the other way is preserved verbatim as any sibling overlay is. ([source](https://opencode.ai/docs/config))

**`tui.jsonc`** -- Documented, and deliberately not owned, for the same reason as opencode.jsonc: it is the second spelling of one file, and owning both would let a target hold two documents that disagree with the product reading one and this provider reporting the other. ([source](https://opencode.ai/docs/tui))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
