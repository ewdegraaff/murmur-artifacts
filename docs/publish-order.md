# Publish order

Run these from the repository root, logged in as the owner with `gh`. The zips and indexes are already built in `release/`, which git ignores. Rebuild them with the commands in each artifact's README if the folder is gone.

This repository has no release workflow on purpose. A workflow that runs on a tag push would race the `gh release create` commands below, rebuild the deliberately wrong cases, and replace the broken index of case 4 with a good one. Every release is made by hand, and the `.wasm` files and zips are never committed.

The order matters. Case 1 and case 5 are published with release immutability on. Cases 2, 3 and 4 are published with it off, so they stay mutable.

## 1. Push the repository

```bash
git add -A && git status --short   # expect no .murmur/, instructions.md or murmur.yaml at the root
git commit -m "feat: five deliberate cases for MAP" 
gh repo create ewdegraaff/murmur-artifacts --public --source . --push 2>&1 || git push -u origin main
gh repo edit ewdegraaff/murmur-artifacts --add-topic murmur-artifacts
gh repo view ewdegraaff/murmur-artifacts --json repositoryTopics,visibility
```

The repository has no licence yet. Pick one and add a `LICENSE` file first if you want one.

## 2. Immutability on, then cases 1 and 5

```bash
gh api -X PUT repos/ewdegraaff/murmur-artifacts/immutable-releases
gh api repos/ewdegraaff/murmur-artifacts/immutable-releases

git tag tool-echo-v0.1.0 && git push origin tool-echo-v0.1.0
gh release create tool-echo-v0.1.0 --draft --verify-tag --title "tool-echo 0.1.0" \
  release/case1/tool-echo-0.1.0.mur.zip release/case1/artifacts-index.json
gh release edit tool-echo-v0.1.0 --draft=false --latest
gh release verify tool-echo-v0.1.0
gh release view tool-echo-v0.1.0 --json isImmutable

git tag skill-release-checklist-v0.1.0 && git push origin skill-release-checklist-v0.1.0
gh release create skill-release-checklist-v0.1.0 --draft --verify-tag --title "skill-release-checklist 0.1.0" \
  release/case5/skill-release-checklist-0.1.0.mur.zip release/case5/artifacts-index.json
gh release edit skill-release-checklist-v0.1.0 --draft=false --latest
```

## 3. Immutability off, then cases 2, 3 and 4

```bash
gh api -X DELETE repos/ewdegraaff/murmur-artifacts/immutable-releases

git tag hook-quiet-v0.1.0 && git push origin hook-quiet-v0.1.0
gh release create hook-quiet-v0.1.0 --draft --verify-tag --title "hook-quiet 0.1.0" \
  release/case2/hook-quiet-0.1.0.mur.zip release/case2/artifacts-index.json
gh release edit hook-quiet-v0.1.0 --draft=false --latest
gh release view hook-quiet-v0.1.0 --json isImmutable

git tag tool-drift-v0.3.0 && git push origin tool-drift-v0.3.0
gh release create tool-drift-v0.3.0 --draft --verify-tag --title "tool-drift 0.3.0" \
  release/case3/tool-drift-0.1.0.mur.zip release/case3/artifacts-index.json
gh release edit tool-drift-v0.3.0 --draft=false --latest

git tag tool-garbled-v0.1.0 && git push origin tool-garbled-v0.1.0
gh release create tool-garbled-v0.1.0 --draft --verify-tag --title "tool-garbled 0.1.0" \
  release/case4/tool-garbled-0.1.0.mur.zip release/case4/artifacts-index.json
gh release edit tool-garbled-v0.1.0 --draft=false --latest=false
```

Case 4 is not the latest release. If it were, its broken index would replace the good one and hide every other case. The latest release stays `tool-drift-v0.3.0`, whose index lists cases 1, 2, 3 and 5.

## 4. Check

```bash
gh release view tool-echo-v0.1.0 --json isImmutable        # true
gh release view hook-quiet-v0.1.0 --json isImmutable       # false
mur install ewdegraaff/tool-echo@0.1.0
mur install ewdegraaff/hook-quiet@0.1.0
mur install ewdegraaff/skill-release-checklist@0.1.0
npx skills add ewdegraaff/murmur-artifacts --skill skill-release-checklist --list
```
