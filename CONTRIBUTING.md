<!--
SPDX-FileCopyrightText: 2019 Antonin Bas

SPDX-License-Identifier: Apache-2.0
-->

You can fork the repo and submit a pull request in GitHub.

### Contributing License

The P4 organizations uses [DCO](https://en.wikipedia.org/wiki/Developer_Certificate_of_Origin) for contributions. Please take a look at our [guidelines](https://github.com/p4lang/governance/wiki/P4-DCO-Guidelines).

### AsciiDoc style checker

The P4Runtime specification is written using [AsciiDoc](https://docs.asciidoctor.org/).
We provide a lint tool to catch basic formatting issues and try to keep the spec uniform. 
The lint tool will be run as part of CI and patches cannot be merged until it returns success. You can
run the lint tool locally with `./tools/asciidoclint.py`.
