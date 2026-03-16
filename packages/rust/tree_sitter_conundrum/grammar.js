/**
 * @file A Rust powered superset of mdx.
 * @author Andrew Mueller (fluster.eyeoh@gmail.com) <fluster.eyeoh@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "conundrum",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
