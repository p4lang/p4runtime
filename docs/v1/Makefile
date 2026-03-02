SPEC=P4Runtime-Spec

ROUGE_STYLE=github
ROUGE_CSS=style

all: ${SPEC}.pdf ${SPEC}.html

folder:
	mkdir -p build

build:

${SPEC}.pdf: ${SPEC}.adoc images folder
	time asciidoctor-pdf -v \
	--failure-level ERROR \
	-a pdf-fontsdir=resources/fonts \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-style=$(ROUGE_STYLE) \
	-D build $<

${SPEC}.html: ${SPEC}.adoc images folder
	time asciidoctor -v \
	--failure-level ERROR \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-css=$(ROUGE_CSS) \
	-D build $<

images:
	soffice --convert-to svg --outdir resources/figs resources/figs/*.odg
	soffice --convert-to png --outdir resources/figs resources/figs/*.odg

build_spec_with_images: images all

clean:
	/bin/rm -rf build resources/figs/*.png resources/figs/*.svg
