# Copyright 2018 Antonin Bas
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
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
