package crashme

import "fmt"

func CheckArg(arg string) {
    if len(arg) == 0 {
        fmt.Println("Usage: need at least one char")
        return
    }
    if arg[0] == 'C' {
        var p *int
        *p = 0 // Crash
    }
    fmt.Println("OK")
}
