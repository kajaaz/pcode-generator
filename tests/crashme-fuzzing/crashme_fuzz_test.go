package crashme

import "testing"

func FuzzCheckArg(f *testing.F) {
    // Provide  seed corpus
    f.Add("A")

    f.Fuzz(func(t *testing.T, input string) {
        CheckArg(input)
    })
}
