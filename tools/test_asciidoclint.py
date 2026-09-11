#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2026 The P4 Language Consortium & Devansh Singh
#
# SPDX-License-Identifier: Apache-2.0

"""Tests for asciidoclint.py. Run with: python3 tools/test_asciidoclint.py"""

import io
import json
import os
import tempfile
import unittest
from contextlib import redirect_stdout

import asciidoclint


DOC_TITLE = "= Example Spec : A Test Document\n"


def run_lint(body, keywords=None):
    asciidoclint.lint_state = asciidoclint.LintState()
    asciidoclint.lint_conf = asciidoclint.LintConf()
    if keywords:
        conf = {"keywords": [{"category": "test", "keywords": keywords}]}
        asciidoclint.lint_conf.build_from(io.StringIO(json.dumps(conf)))

    with tempfile.NamedTemporaryFile(
            mode="w", suffix=".adoc", delete=False) as f:
        f.write(DOC_TITLE)
        f.write(body)
        path = f.name

    try:
        out = io.StringIO()
        with redirect_stdout(out):
            asciidoclint.process_one(path)
        return out.getvalue(), asciidoclint.lint_state.errors_cnt
    finally:
        os.unlink(path)


class ContextAfterTitleTest(unittest.TestCase):

    def test_long_line_after_title_is_flagged(self):
        body = "x" * 100 + "\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 1)

    def test_long_line_before_title_is_ignored(self):
        asciidoclint.lint_state = asciidoclint.LintState()
        asciidoclint.lint_conf = asciidoclint.LintConf()
        with tempfile.NamedTemporaryFile(
                mode="w", suffix=".adoc", delete=False) as f:
            f.write(":some-long-attribute-value: " + ("y" * 100) + "\n")
            f.write(DOC_TITLE)
            path = f.name
        try:
            with redirect_stdout(io.StringIO()):
                asciidoclint.process_one(path)
            self.assertEqual(asciidoclint.lint_state.errors_cnt, 0)
        finally:
            os.unlink(path)

    def test_unbackticked_keyword_is_flagged(self):
        body = "The server must return NOT_FOUND in this case\n"
        _, errors = run_lint(body, keywords=["NOT_FOUND"])
        self.assertEqual(errors, 1)

    def test_backticked_keyword_is_not_flagged(self):
        body = "The server must return `NOT_FOUND` in this case\n"
        _, errors = run_lint(body, keywords=["NOT_FOUND"])
        self.assertEqual(errors, 0)


class ContextSkipHeadingsTest(unittest.TestCase):

    def test_long_heading_is_skipped(self):
        body = ("=== " + ("A very long section heading " * 5)).rstrip() + "\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 0)

    def test_long_line_after_heading_is_still_flagged(self):
        body = "=== A Short Heading\n" + ("z" * 100) + "\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 1)

    def test_hash_prefixed_line_is_not_treated_as_heading(self):
        body = "#" + ("a" * 100) + "\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 1)


class ContextSkipBlocksListingTest(unittest.TestCase):

    def test_long_line_inside_listing_block_is_skipped(self):
        body = "----\n" + ("x" * 100) + "\n----\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 0)

    def test_unbackticked_keyword_inside_listing_block_is_skipped(self):
        body = "----\nmessage WriteRequest {\n}\n----\n"
        _, errors = run_lint(body, keywords=["WriteRequest"])
        self.assertEqual(errors, 0)

    def test_long_line_after_listing_block_is_still_flagged(self):
        body = "----\ncode inside\n----\n" + ("y" * 100) + "\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 1)

    def test_long_table_row_is_skipped(self):
        body = "|===\n| " + ("a" * 100) + "\n|===\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 0)

    def test_long_block_attribute_line_is_skipped(self):
        body = "[.center,cols=\"2\"," + ("a" * 80) + "]\n"
        _, errors = run_lint(body)
        self.assertEqual(errors, 0)


class TrailingWhitespaceTest(unittest.TestCase):

    def test_trailing_whitespace_is_flagged(self):
        _, errors = run_lint("hello \n")
        self.assertEqual(errors, 1)

    def test_trailing_whitespace_on_last_line_without_newline_is_flagged(self):
        _, errors = run_lint("hello ")
        self.assertEqual(errors, 1)

    def test_last_line_without_newline_and_without_trailing_whitespace_is_not_flagged(self):
        _, errors = run_lint("hello")
        self.assertEqual(errors, 0)

    def test_blank_line_is_not_flagged(self):
        _, errors = run_lint("\n")
        self.assertEqual(errors, 0)


class ShortLineTest(unittest.TestCase):

    def test_short_line_is_not_flagged(self):
        _, errors = run_lint("This is a short, unremarkable line.\n")
        self.assertEqual(errors, 0)


if __name__ == "__main__":
    unittest.main()
