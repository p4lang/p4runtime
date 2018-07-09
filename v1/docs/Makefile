
SPEC=P4Runtime-Spec

all: html pdf

html: ${SPEC}.mdk p4.json assets/*
	madoko -vv --png --odir=build ${SPEC}.mdk

pdf: ${SPEC}.mdk p4.json assets/*
	madoko --pdf -vv --png --odir=build $<

clean:
	${RM} -rf build
