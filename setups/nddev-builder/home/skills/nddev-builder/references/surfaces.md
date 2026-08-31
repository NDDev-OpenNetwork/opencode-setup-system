# What This Harness Owns

Generated from `references/opencode-baseline.json` by
`tools/build_nddev_builder.py`. Do not edit: the next render overwrites
it, and the baseline is where a correction belongs.

Every row below was decided by a source, and the source is named. Where
this file and the binary disagree, the binary is right -- ask it with
`opencode-setup-system provider-info`.

**Configuration home**: `~/.config/opencode`
**Environment override**: `OPENCODE_CONFIG_DIR`

## The configuration file

`opencode.json` is **jsonc**, and the parser accepts comments.
The vendor publishes a schema at <https://opencode.ai/config.json>, and `tools/validate_setup_schemas.py` checks every shipped file that names it.

**JSONC**, at both spellings -- line comments, block comments and trailing commas are all accepted, and the product's own bundle uses a `jsonc-parser` with `formattingOptions`. Strict JSON is a subset, so what this repository writes is valid; the distinction matters for a file a *person* has edited, which a backup will capture with its comments intact.

## Owned surfaces

| path | kinds | shape | decided by | exercised by |
|---|---|---|---|---|
| `AGENTS.md` | instruction | file | <https://opencode.ai/docs/rules> | read its bytes |
| `opencode.json` | setting | file | <https://opencode.ai/docs/config> | **ran it** |
| `skills` | skill | directory | <https://opencode.ai/docs/skills> | **ran it** |
| `agents` | agent | directory | <https://opencode.ai/docs/agents> | **ran it** |
| `commands` | command | directory | <https://opencode.ai/docs/commands> | **ran it** |
| `plugins` | plugin | directory | <https://opencode.ai/docs/plugins> | read its bytes |
| `tui.json` | *(routes no kind)* | file | <https://opencode.ai/docs/tui> | *nothing — a page* |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **4 run**, **2 read from the product's own bytes**, and **1 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## A second target: `target_scope: user_root`

Rooted at `~/.agents`, which is **not** this product's configuration
home. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root rather than to the home
above -- writing the root into the path again would nest it twice.

| path | routes | shape | decided by | exercised by |
| --- | --- | --- | --- | --- |
| `skills` | skill | directory | <https://opencode.ai/docs/skills/, and measured by running the pinned 1.18.25 product with `debug skill` against a temporary HOME, 2026-08-29> | **ran it** |

**Under a scope the namespace is the permission and the recorded
files are the inventory.** A root like this one is read by several
products at once, so `remove`, the capture and a restore all act on
the files this provider recorded writing -- never on the namespace
whole, which would take or revert a neighbour's work.

## Considered and not owned

14 rows. Each records what was searched, so the next reader does not repeat the search:

- **`opencode.jsonc`** — Documented, and deliberately not owned. OpenCode reads either spelling; owning both would let a target hold two documents that disagree, with the product picking one and this provider reporting the other. Owning one keeps the answer single, and a target configured the other way is preserved verbatim as any sibling overlay is.
- **`tui.jsonc`** — Documented, and deliberately not owned, for the same reason as opencode.jsonc: it is the second spelling of one file, and owning both would let a target hold two documents that disagree with the product reading one and this provider reporting the other.
- **`NDDEV-OPENCODE-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.opencode-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`.gitignore`** — The product writes this into its configuration home on first run, listing `node_modules`, `package.json`, `package-lock.json`, `bun.lock` and itself -- it treats the home as a place a package manager might run. Measured 2026-08-28 by launching the product through this provider. Not owned: nothing here projects a `.gitignore`, and a file the product rewrites on its own schedule is not a surface a setup can promise to restore.
- **`$HOME/.claude/skills`** — OpenCode also reads Claude Code's skills directory for compatibility -- `.claude/skills/<name>/SKILL.md` is a path literal in the pinned binary, and the vendor lists it as *Global Claude-compatible*. Another product's home, never this provider's to own, and recorded because claude-setup-system owns `skills` there.
- **`opencode-runtime-state`** — One row for what the product writes **outside its configuration home entirely**, because it writes to three other roots and none of them had a row.
- **`agent`** — Opencode accepts the singular and the plural spelling of this directory. Its own embedded reference, carried in the pinned 1.18.25 binary, writes the global row as `~/.config/opencode/agent(s)/<name>.md` and the project row as `.opencode/agent/<name>.md` or `.opencode/agents/<name>.md`. This provider writes and owns the plural only. Owning both would let one target hold two definitions of the same component that disagree, with the product reading one and this provider reporting the other, and which of the two wins where both exist is not documented -- so a target holding the singular is reported rather than resolved. Recorded here because without a row the next reader repeats the search, and because a directory the product reads and this provider does not own is exactly what this block is for.
- **`command`** — Opencode accepts the singular and the plural spelling of this directory. Its own embedded reference, carried in the pinned 1.18.25 binary, writes the global row as `~/.config/opencode/command(s)/<name>.md` and the project row as `.opencode/command/<name>.md` or `.opencode/commands/<name>.md`. This provider writes and owns the plural only. Owning both would let one target hold two definitions of the same component that disagree, with the product reading one and this provider reporting the other, and which of the two wins where both exist is not documented -- so a target holding the singular is reported rather than resolved. Recorded here because without a row the next reader repeats the search, and because a directory the product reads and this provider does not own is exactly what this block is for.
- **`skill`** — Opencode accepts the singular and the plural spelling of this directory. Its own embedded reference, carried in the pinned 1.18.25 binary, writes the global row as `~/.config/opencode/skill(s)/<name>/SKILL.md` and the project row as `.opencode/skill/<name>/SKILL.md` or `.opencode/skills/<name>/SKILL.md`. This provider writes and owns the plural only. Owning both would let one target hold two definitions of the same component that disagree, with the product reading one and this provider reporting the other, and which of the two wins where both exist is not documented -- so a target holding the singular is reported rather than resolved. Recorded here because without a row the next reader repeats the search, and because a directory the product reads and this provider does not own is exactly what this block is for.
- **`plugin`** — Opencode accepts the singular and the plural spelling of this directory. Its own embedded reference, carried in the pinned 1.18.25 binary, writes the global row as `~/.config/opencode/plugin(s)/<name>/` and the project row as `.opencode/plugin/<name>/` or `.opencode/plugins/<name>/`. This provider writes and owns the plural only. Owning both would let one target hold two definitions of the same component that disagree, with the product reading one and this provider reporting the other, and which of the two wins where both exist is not documented -- so a target holding the singular is reported rather than resolved. Recorded here because without a row the next reader repeats the search, and because a directory the product reads and this provider does not own is exactly what this block is for.
- **`managed-config`** — Not a path in the target, and named without an extension for that reason: the managed configuration directory is a **system** path, one per operating system, and every recorded path here is relative to the target.
- **`mcp_config.json`** — MCP servers are the `mcp` key inside `opencode.json` -- `{"mcp": {"<name>": {"type": "local", "command": [...]}}}` -- confirmed on the vendor's MCP page 2026-08-29 and in the product's own built-in `customize-opencode` skill. That file is owned here and written and restored whole, so MCP is covered by the `setting` kind. **A key inside a file is not a projection surface.** No separate MCP file exists under the config home.
- **`hooks.json`** — **Hooks are functions a plugin module exports**, not a file. The vendor's plugin page, read 2026-08-29: *"A plugin is a JavaScript/TypeScript module that exports one or more plugin functions. Each function receives a context object and returns a hooks object."* The names -- `tool.execute.before`, `session.created`, `permission.asked` and the rest -- are keys of that returned object. There is no `hooks.json` and no `hooks` key in `opencode.json`, so a hook reaches this product through `plugins/`, which is owned and routes `plugin`.
