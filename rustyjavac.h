#ifndef RUSTYJAVAC_NATIVE_H
#define RUSTYJAVAC_NATIVE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Compile Java source files with RustyJavaC.
 *
 * @param source_files  Array of null-terminated source file paths.
 * @param source_count  Number of entries in source_files.
 * @param output_dir    Output directory for .class files (nullable; defaults to ".").
 * @param java_version  Java version number (e.g. 25 for Java 25).
 * @return 0 on success, non-zero on failure.
 */
int32_t rustyjavac_compile(
    const char **source_files,
    int32_t source_count,
    const char *output_dir,
    int32_t java_version
);

#ifdef __cplusplus
}
#endif

#endif /* RUSTYJAVAC_NATIVE_H */
