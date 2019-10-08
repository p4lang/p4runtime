You can fork the repo and submit a pull request in Github.

### Apache CLA

All developers must have signed the [P4.org](http://p4.org) CLA.

### Madoko style checker

The P4Runtime specification is written using
[Madoko](http://madoko.org/reference.html). We provide a lint tool to catch
basic formatting issues and try to keep the spec uniform. The lint tool will be
run as part of CI and patches cannot be merged until it returns success. You can
run the lint tool locally with `./tools/madokolint.py`.
