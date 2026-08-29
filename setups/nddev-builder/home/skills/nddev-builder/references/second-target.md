# The second target this harness owns

## `target_scope: user_root`, rooted at `~/.agents`

**`~/.agents` is not this product's configuration home.** It is a
different target, reached by a consumer naming the scope on the
request, and every path below is relative to that root rather than
to the home -- writing the root into the path again would nest it
twice, which is a mistake this estate has made and shipped.

| path | routes | decided by | exercised by |
|---|---|---|---|
| `skills` | skill | <https://opencode.ai/docs/skills/> | **ran it** |
### `skills`, as measured

**Run, not read.** The pinned `opencode-linux-x64-1.18.25.tgz` was fetched, its digest checked against the artifact table, and the binary driven with `opencode debug skill` -- a credential-free command that prints the resolved skill list as JSON. A `SKILL.md` at `$HOME/.agents/skills/nddev-user-root-probe/` is listed with its full path.

**With a control.** A second skill at `$HOME/.agents-not-a-root/skills/` -- a sibling root no page names -- is absent from the same listing, so the product reads the documented root rather than scanning `$HOME` broadly.

The product corroborates it in its own words: its built-in `customize-opencode` skill tabulates *External skills (auto-loaded): `~/.claude/skills/<name>/SKILL.md`, `~/.agents/skills/<name>/SKILL.md`*, and names `OPENCODE_DISABLE_EXTERNAL_SKILLS` and `OPENCODE_DISABLE_CLAUDE_CODE_SKILLS` as the switches that turn those two scans off.

The one root in this estate that belongs to a convention rather than to a product. `$HOME/.agents/skills` is a *sibling* of this product's configuration home, not a child, so nothing declared against this provider's own target can reach it -- that is what `user_root` exists for.

**Owning a shared root, and the reason this record used to decline it.** Five of the seven products read this root, and the decline said: *a namespace is removed whole, so a second declaration would make either provider's remove take the other's skills.* That sentence was true when it was written and stopped being true when `written_paths` shipped -- `remove` under this scope takes the files this provider recorded writing and refuses rather than widening when it cannot read the record, and each harness carries its own state file, so they coexist under one root. The reason was not re-read when the thing it described changed.

Relative to this scope's own root the path is `skills`, not `.agents/skills`: the root is what the scope names, and writing it into the path again would put the skills at `~/.agents/.agents/skills`.


**A setup cannot carry one of these.** A setup is installed into one
target and its payload is relative to that target, so a component
for this scope is installed by the consumer against that root -- not
by a setup aimed at the configuration home. If you are looking for
where to put one by hand, it is the path above joined to the root
above, and nowhere under the home.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

