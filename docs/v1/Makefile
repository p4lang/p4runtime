SPEC=P4Runtime-Spec

ROUGE_STYLE=github
ROUGE_CSS=style

all: ${SPEC}.pdf ${SPEC}.html

build:
	mkdir -p build

${SPEC}.pdf: ${SPEC}.adoc
	time asciidoctor-pdf -v \
	-a pdf-fontsdir=resources/fonts \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-style=$(ROUGE_STYLE) $<

${SPEC}.html: ${SPEC}.adoc
	time asciidoctor -v \
	-r asciidoctor-mathematical \
	-r asciidoctor-bibtex \
	-r asciidoctor-lists \
	-a rouge-css=$(ROUGE_CSS) $<

clean:
	/bin/rm -f ${SPEC}.pdf ${SPEC}.html