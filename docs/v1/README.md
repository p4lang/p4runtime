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
You can use the [local installation](https://github.com/p4lang/p4-spec/blob/main/p4-16/spec/install-asciidoctor-linux.sh)
method.

### MacOS

We do not yet have instructions for generating PDF and HTML from AsciiDoc source on macOS. You are welcome to contribute documentation for how to do so if you find instructions that work.

### Windows

We do not yet have instructions for generating PDF and HTML from AsciiDoc source on Windows. You are welcome to contribute documentation for how to do so if you find instructions that work.
