package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"time"

	"github.com/hyphacoop/go-dasl/cid"
	"github.com/hyphacoop/go-dasl/drisl"
)

type Output struct {
	ImplID         string  `json:"impl_id"`
	Accepted       bool    `json:"accepted"`
	ErrorClass     string  `json:"error_class"`
	CanonicalBytes *string `json:"canonical_bytes"`
	TimeMS         int64   `json:"time_ms"`
}

func main() {
	defer func() {
		if r := recover(); r != nil {
			os.Exit(1)
		}
	}()

	format := os.Getenv("DASL_FORMAT")
	if format == "" {
		format = "drisl1"
	}

	input, _ := io.ReadAll(os.Stdin)
	start := time.Now()

	switch format {
	case "cid":
		parsed, err := cid.NewCidFromBytes(input)
		if err != nil {
			reject("go-dasl", "decode_error", start)
			return
		}
		bytes := parsed.Bytes()
		hexBytes := hex.EncodeToString(bytes)
		accept("go-dasl", hexBytes, start)
		return
	case "car":
		reject("go-dasl", "unsupported_format", start)
		return
	default:
		var v any
		if err := drisl.Unmarshal(input, &v); err != nil {
			reject("go-dasl", "decode_error", start)
			return
		}
		canonical, err := drisl.Marshal(v)
		if err != nil {
			reject("go-dasl", "encode_error", start)
			return
		}
		hexBytes := hex.EncodeToString(canonical)
		accept("go-dasl", hexBytes, start)
	}
}

func accept(impl string, hexBytes string, start time.Time) {
	write(Output{
		ImplID:         impl,
		Accepted:       true,
		ErrorClass:     "none",
		CanonicalBytes: &hexBytes,
		TimeMS:         time.Since(start).Milliseconds(),
	})
}

func reject(impl string, class string, start time.Time) {
	write(Output{
		ImplID:         impl,
		Accepted:       false,
		ErrorClass:     class,
		CanonicalBytes: nil,
		TimeMS:         time.Since(start).Milliseconds(),
	})
}

func write(out Output) {
	b, _ := json.Marshal(out)
	fmt.Printf("%s", string(b))
}
