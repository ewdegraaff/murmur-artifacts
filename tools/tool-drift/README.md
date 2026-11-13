# tool-drift

A copy of tool-echo. It is the test source's case 3: a release whose tag version differs from the version in its file name and manifest.

| Place | Version |
|---|---|
| Manifest in the zip | 0.1.0 |
| File name | 0.1.0 |
| Release tag | 0.3.0 |

`mur index` refuses a file name that differs from the packed manifest, so the drift is in the tag. A release workflow would refuse this shape, so this repository has none and every release is published by hand. See `docs/publish-order.md`.
