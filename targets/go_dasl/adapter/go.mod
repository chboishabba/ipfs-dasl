module go_dasl_adapter

go 1.24.0

require github.com/hyphacoop/go-dasl v0.0.0

require (
	github.com/hyphacoop/cbor/v2 v2.0.0-20251007204234-2a4fa83e606e // indirect
	github.com/klauspost/cpuid/v2 v2.0.9 // indirect
	github.com/x448/float16 v0.8.4 // indirect
	lukechampine.com/blake3 v1.4.1 // indirect
)

replace github.com/hyphacoop/go-dasl => ../src
