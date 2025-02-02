# Copyright 2018 Antonin Bas
# SPDX-License-Identifier: Apache-2.0

SPEC=P4Runtime-Spec

ROUGE_STYLE=github
ROUGE_CSS=style

all: ${SPEC}.pdf ${SPEC}.html

folder:
	mkdir -p build

build:

${SPEC}.pdf: ${SPEC}.adoc images folder
	time asciidoctor-pdf -v \
	-a pdf-fontsdir=resources/fonts \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-style=$(ROUGE_STYLE) \
	-D build $<

${SPEC}.html: ${SPEC}.adoc images folder
	time asciidoctor -v \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-css=$(ROUGE_CSS) \
	-D build $<

images:
	soffice --convert-to svg --outdir resources/figs resources/figs/*.odg > /dev/null 2>&1
	soffice --convert-to png --outdir resources/figs resources/figs/*.odg > /dev/null 2>&1

build_spec_with_images: images all

clean:
	/bin/rm -rf build resources/figs/*.png resources/figs/*.svg
