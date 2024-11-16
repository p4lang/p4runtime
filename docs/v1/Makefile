
SPEC=P4Runtime-Spec

ROUGE_STYLE=github
ROUGE_CSS=style

all: ${SPEC}.pdf ${SPEC}.html

build:
	mkdir -p build

# Note: Brute-force image rendering; could use more precise file-by-file make rules
images: build resources/figs/*.odg
	soffice --convert-to svg --outdir build resources/figs/*.odg > /dev/null 2>&1
	soffice --convert-to png --outdir build resources/figs/*.odg > /dev/null 2>&1

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
	/bin/rm -f ${SPEC}.pdf ${SPEC}.html resources/figs/stem-*.png
