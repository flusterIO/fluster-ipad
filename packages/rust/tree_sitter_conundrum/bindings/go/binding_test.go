package tree_sitter_conundrum_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_conundrum "github.com/flusterio/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_conundrum.Language())
	if language == nil {
		t.Errorf("Error loading Conundrum grammar")
	}
}
