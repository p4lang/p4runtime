# P4Runtime Specification Version 1

This directory contains the sources for generating the official P4Runtime
specification document.

# Markup version

The markup version uses Madoko (https://www.madoko.net) to produce
HTML and PDF versions of the documentation. Pre-built versions of the
documentation are available at **[TODO]**


Files:
- `P4Runtime-spec.mdk` is the main file. 
- `assets/*.png` Figures
- `Makefile` builds documentation in the build subdirectory
- `p4.json` is providing custom syntax highlighting for P4. It is a rip off from
  the cpp.json provided by Madoko (the "extend" clause does not work, my version
  of Madoko asks for a "tokenizer" to be defined). Style customization for each
  token can be done using CSS style attributes (see `token.keyword` in
  `P4Runtime-spec.mdk`).

## Building

### Linux
```
sudo apt-get install nodejs
sudo npm install madoko -g
make [all | html | pdf ]
```
In particular (on Ubuntu 16.04 at least), don't try `sudo apt-get install npm`
because `npm` is already included and this will yield a bunch of confusing error
messages from `apt-get`.

### MacOS

We use the [local
installation](http://research.microsoft.com/en-us/um/people/daan/madoko/doc/reference.html#sec-installation-and-usage)
method. For Mac OS, I installed node.js using Homebrew and then Madoko using
npm:
```
brew install node.js
npm install madoko -g
```
Note that to build the PDF you need a functional TeX version installed.

If you check out the ```gh-pages``` branch of this repository, the
following two files can be found in the root directory.  You may
install them on a Mac using Font Book:

```
UtopiaStd-Regular.otf
luximr.ttf
```

### Windows

You need to install miktex [http://miktex.org/], madoko
[https://www.madoko.net/] and node.js [https://nodejs.org/en/].  To
build you can invoke the make.bat script.


# TODO
## Formating Fixups TODO
Following is a list of outstanding document formatting chores.  Some are left
over as a result of converting from Google Docs to Madoko for the initial
1.0.0-rc1 version. Others are Madoko-PDF rendering issues which may require
hand-tweaking or workarounds.

* Correct internal links e.g. to section references. Reference tags need to be
  added to sections, and the links to them need to be replaced.
* Surround `tokens` with backticks. All the Courier font formatting in the gDocs
  was lost.

## Content TODO
Following are major content items which are missing or incomplete:
*  Section 17. P4Runtime Versioning
*  Section 18. Extending P4Runtime for non-PSA architectures
*  Section 19. Lifetime of a session