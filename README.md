# RustyJavaC Native
Rust FFI bindings for [Rusty-JavaC](https://github.com/Eatgrapes/Rusty-JavaC). Compiles to a native shared library (`.dll` / `.so` / `.dylib`) for JVM integration via [Panama FFM](https://openjdk.org/jeps/454) or JNA.

## Quick Start

### Build

```bash
git clone https://github.com/CoffeePopStudio/RustyJavaC-Native.git
cd RustyJavaC-Native
cargo build --release
```

Produces:
- `target/release/rustyjavac_native.dll` (Windows)
- `target/release/librustyjavac_native.so` (Linux)
- `target/release/librustyjavac_native.dylib` (macOS)

### C API

```c
int32_t rustyjavac_compile(
    const char **source_files,
    int32_t source_count,
    const char *output_dir,
    int32_t java_version
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `source_files` | `const char**` | Null-terminated array of source file paths |
| `source_count` | `int32_t` | Number of entries |
| `output_dir` | `const char*` | Output directory (nullable) |
| `java_version` | `int32_t` | Java version, e.g. 25 |
| **Returns** | `int32_t` | 0 on success, non-zero on failure |

### Panama FFM Usage (Java 25+)

```java
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

var linker = Linker.nativeLinker();
var lib = SymbolLookup.libraryLookup("rustyjavac_native", Arena.global());

var compileHandle = linker.downcallHandle(
    lib.find("rustyjavac_compile").orElseThrow(),
    FunctionDescriptor.of(ValueLayout.JAVA_INT,
        ValueLayout.ADDRESS,  // const char**
        ValueLayout.JAVA_INT, // source_count
        ValueLayout.ADDRESS,  // const char*
        ValueLayout.JAVA_INT  // java_version
    )
);

try (var arena = Arena.ofConfined()) {
    var argv = arena.allocateFrom(ValueLayout.ADDRESS,
        arena.allocateFrom("HelloWorld.java"),
        arena.allocateFrom("Utils.java"));
    var outDir = arena.allocateFrom("build/classes");

    int rc = (int) compileHandle.invoke(argv, 2, outDir, 25);
    if (rc != 0) throw new RuntimeException("compilation failed");
}
```

## Requirements

- Rust 1.85+
- Cargo

## License

[MIT](LICENSE)
