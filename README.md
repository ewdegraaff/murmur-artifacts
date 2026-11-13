# murmur-artifacts

The test source for MAP, the Murmur Artifact Proxy. Each artifact here is one deliberate case that the listing bar and the trust display must handle.

| Case | Artifact | What it shows |
|---|---|---|
| 1 | `tool-echo` | A tool in an immutable release with an attestation |
| 2 | `hook-quiet` | A hook in a mutable release |
| 3 | `tool-drift` | A release tagged `tool-drift-v0.3.0` for a file and manifest at 0.1.0 |
| 4 | `tool-garbled` | A release whose `artifacts-index.json` does not parse |
| 5 | `skill-release-checklist` | A skill at `skills/<name>/SKILL.md` that installs through `mur` and `npx skills` |

Nothing here is meant for real use. The publishing order is in [docs/publish-order.md](docs/publish-order.md).

```bash
mur install ewdegraaff/tool-echo@0.1.0
npx skills add ewdegraaff/murmur-artifacts --skill skill-release-checklist
```
