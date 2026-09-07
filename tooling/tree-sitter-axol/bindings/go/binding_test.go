package tree_sitter_axolotlgrammer_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_axolotlgrammer "github.com/tree-sitter/tree-sitter-axolotlgrammer/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_axolotlgrammer.Language())
	if language == nil {
		t.Errorf("Error loading Axolotl Grammer grammar")
	}
}
