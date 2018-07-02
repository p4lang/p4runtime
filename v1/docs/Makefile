
SPEC=P4Runtime-Spec

all: build/${SPEC}.pdf

html:
	madoko -vv --png --odir=build ${SPEC}.mdk

build/${SPEC}.pdf: ${SPEC}.mdk
	madoko --pdf -vv --png --odir=build $<

clean:
	${RM} -rf build
