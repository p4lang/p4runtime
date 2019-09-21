#!/usr/bin/env python3

# Copyright 2019 VMware, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#


# DISCLAIMER: This is a work in progress. This linter was written specifically
# for the P4Runtime specification document and may not be useful for other
# Madoko documents, as it may be making some assumptions as to how the document
# was written.

# TODO: handle Madoko includes (we do not use them for the P4Runtime spec)?


import argparse
from collections import namedtuple
import os.path
import re
import sys


LINE_WRAP_LENGTH = 80


parser = argparse.ArgumentParser(description='Lint tool for Madoko code')
parser.add_argument('files', metavar='FILE', type=str, nargs='+',
                    help='Input files')


class MadokoFmtError(Exception):
    def __init__(self, filename, lineno, description):
        self.filename = filename
        self.lineno = lineno
        self.description = description

    def __str__(self):
        return "Unexpected Madoko code in file {} at line {}: {}".format(
            self.filename, self.lineno, self.description)


class LintState:
    def __init__(self):
        self.errors_cnt = 0

    def error(self, filename, lineno, line, description):
        # TODO: print line later?
        print("Error in file {} at line {}: {}.".format(filename, lineno, description))
        self.errors_cnt += 1


lint_state = LintState()


class Context:
    """A context is an object that is used to determine whether a specific "checker" (check_*
    method) should visit a given line."""

    def enter(self, line, filename, lineno):
        """Called before visiting a line.
        Returns True iff the checker should visit the given line.
        """
        return True

    def exit(self, line, filename, lineno):
        """Called after visiting a line."""
        pass


class ContextSkipBlocks(Context):
    """A context used to only visit Madoko code outside of blocks."""

    Block = namedtuple('Block', ['num_tildes', 'name'])

    def __init__(self):
        self.p_block = re.compile('^ *(?P<tildes>~+) *(?:(?P<cmd>Begin|End)(?: +))?(?P<name>\w+)?')
        self.blocks_stack = []

    def enter(self, line, filename, lineno):
        m = self.p_block.match(line)
        if m:
            num_tildes = len(m.group("tildes"))
            has_begin = m.group("cmd") == "Begin"
            has_end = m.group("cmd") == "End"
            blockname = m.group("name")

            if has_begin:
                self.blocks_stack.append(self.Block(num_tildes, blockname))
                return False
            if has_end:
                if not self.blocks_stack:
                    raise MadokoFmtError(filename, lineno, "Block end line but no block was begun")
                expected = self.blocks_stack.pop()
                if num_tildes != expected.num_tildes or blockname != expected.name:
                    raise MadokoFmtError(
                        filename, lineno,
                        "Block end line does not match last visited block begin line")
                return False
            if blockname is None:
                if not self.blocks_stack:
                    raise MadokoFmtError(filename, lineno, "Block end line but no block was begun")
                expected = self.blocks_stack.pop()
                if num_tildes != expected.num_tildes:
                    raise MadokoFmtError(
                        filename, lineno,
                        "Block end line does not match last visited block begin line")
                return False
            self.blocks_stack.append(self.Block(num_tildes, blockname))
            return False
        if self.blocks_stack:
            return False
        return True


# TODO: would "skip metadata" be more generic?
class ContextAfterTitle(Context):
    """A context used to visit only Madoko code after the [TITLE] block element.
    """

    def __init__(self, *args):
        self.title_found = False
        self.p_title = re.compile('^ *\[TITLE\] *$')

    def enter(self, line, filename, lineno):
        if self.title_found:
            return True
        self.title_found = self.p_title.match(line) is not None
        return False


class ContextSkipHeadings(Context):
    """A context used to skip headings (lines starting with #)."""

    def __init__(self, *args):
        self.p_headings = re.compile('^ *#')

    def enter(self, line, filename, lineno):
        return self.p_headings.match(line) is None


class ContextCompose(Context):
    """A special context used to combine an arbitrary number of contexts."""

    def __init__(self, *args):
        self.contexts = list(args)

    def enter(self, line, filename, lineno):
        res = True
        for c in self.contexts:
            # we use a short-circuit on purpose, if a context returns False we do not even enter
            # subsequent contexts. This has some implications on how contexts are used.
            res = res and c.enter(line, filename, lineno)
        return res

    def exit(self, line, filename, lineno):
        for c in self.contexts:
            c.exit(line, filename, lineno)


def foreach_line(path, context, fn):
    """Iterate over every line in the file. For each line, call fn iff the enter method of the
    provided context returns True."""
    lineno = 1
    with open(path, 'r') as f:
        for line in f:
            if context.enter(line, path, lineno):
                fn(line, lineno)
            lineno += 1
            context.exit(line, path, lineno)


def check_line_wraps(path):
    def check(line, lineno):
        if "http" in line:  # TODO: we can probably do better than this
            return
        if len(line) > LINE_WRAP_LENGTH + 1:  # +1 for the newline characted
            lint_state.error(path, lineno, line,
                             "is more than {} characters long".format(LINE_WRAP_LENGTH))

    foreach_line(path,
                 ContextCompose(ContextAfterTitle(), ContextSkipBlocks(), ContextSkipHeadings()),
                 check)


def check_trailing_whitespace(path):
    def check(line, lineno):
        if len(line) >= 2 and line[-2].isspace():
            lint_state.error(path, lineno, line, "trailing whitespace")

    foreach_line(path, Context(), check)

def check_predefined_abbreviations(path):
    abbreviations = {
        'e.g.': '&eg;',
        'i.e.': '&ie;',
        'et al.': '&etal;',
    }

    def check(line, lineno):
        for k, v in abbreviations.items():
            if k in line:
                lint_state.error(path, lineno, line,
                                 "contains '{}', use '{}' instead".format(k, v))

    foreach_line(path, ContextCompose(ContextAfterTitle(), ContextSkipBlocks()), check)


def process_one(path):
    check_line_wraps(path)
    check_predefined_abbreviations(path)
    check_trailing_whitespace(path)


def main():
    args = parser.parse_args()

    for f in args.files:
        if not os.path.isfile(f):
            print("'{}' is not a valid file path".format(f))
            sys.exit(1)
        _, ext = os.path.splitext(f)
        if ext != ".mdk":
            print("'{}' does not have an .mdk extension")
            sys.exit(1)

    for f in args.files:
        try:
            process_one(f)
        except MadokoFmtError as e:
            print(e)

    errors_cnt = lint_state.errors_cnt
    print("**********")
    print("Errors found: {}".format(errors_cnt))
    rc = 0 if errors_cnt == 0 else 2
    sys.exit(rc)


if __name__ == '__main__':
    main()
