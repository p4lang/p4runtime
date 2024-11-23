# P4Runtime Specification Version 1

This directory contains the sources for generating the official P4Runtime
specification document.

# Markup version

The markup version uses AsciiDoc (https://docs.asciidoctor.org/) to produce
HTML and PDF versions of the documentation. Pre-built versions of the
documentation are available on the [P4.org specifications
page](https://p4.org/specs).


Files:
- `P4Runtime-Spec.adoc` is the main file. 
- resources: 
  - figs
    - `*.odg` - OfficeLibre source drawing file used to export images. These are
    bulk-rendered at build time into .svg and .png images via `soffice`
    command-line (required in build environment)
  - fonts 
      - `*.ttf` - Type font source file used to export fonts.
  - theme: 
      - `*.yaml` - Describes how PDF P4Runtime specification will be displayed.
      - `*.css`  - Describes how HTML P4Runtime specification will displayed.
      - `*.bib`  - Bibliography file that contains a list of bibliographical item, such as articles, books, and theses.
- `Makefile` builds documentation in the build subdirectory


## Building

The easiest way to render the AsciiDoc specification documentation is to use the
`p4lang/p4rt-asciidoc:latest` Docker` image:

    docker run -v `pwd`/docs/v1:/usr/src/p4-spec p4lang/p4rt-asciidoc:latest make

### Linux
```
rvm install ruby-3.3.1
rvm use 3.3.1
gem install asciidoctor
gem install asciidoctor-pdf
gem install asciidoctor-bibtex
gem install asciidoctor-mathematical
gem install prawn-gmagick
gem install rouge
gem install asciidoctor-bibtex
gem install asciidoctor-lists
gem install prawn-gmagick
make 
```
You can use the [local
installation](https://github.com/p4lang/p4-spec/blob/main/p4-16/spec/install-asciidoctor-linux.sh)
method.

### MacOS

We use the [local
installation](https://github.com/p4lang/p4-spec/blob/main/p4-16/spec/install-asciidoctor-linux.sh)
method. For Mac OS you can install AsciiDoc using Homebrew via:
```
brew install asciidoctor
```

### Windows

You need to install chocolatey [https://chocolatey.org/] or rubyInstaller [https://rubyinstaller.org/downloads/]. 
Once you’ve installed Ruby , open a terminal and type:
```
 gem install asciidoctor
```
