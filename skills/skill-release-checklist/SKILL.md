---
name: skill-release-checklist
description: "A short checklist for releasing a new version of a Murmur artifact from a murmur-artifacts repository: bump the version in the manifest, build, check the file name, tag as <name>-v<version> and read the release back. Use when releasing or re-releasing an artifact and you want to avoid a version that differs between file, manifest and tag."
metadata:
  version: "0.1.0"
---
# Release checklist

Work through these steps in order.

1. Bump `version` in the artifact's `murmur.yaml`, or `metadata.version` in a skill's front block. A released version is never changed, so every change is a new version.
2. Run `mur build <folder> --output release/`. The file is named `<name>-<version>.mur.zip`.
3. Check that the file name, the packed `murmur.yaml` and the tag you are about to push carry the same version.
4. Tag the commit `<name>-v<version>` and push the tag. The release workflow does the rest.
5. Read it back with `gh release view <name>-v<version>` and `mur install <scope>/<name>@<version>`.

## In a Murmur capsule

The runtime offers this skill as a tool with no parameters. It returns this body.
