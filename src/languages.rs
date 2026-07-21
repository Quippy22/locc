use std::path::Path;

pub fn detect(path: &Path) -> Option<(&'static str, (u8, u8, u8))> {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" => Some(("Rust", (222, 165, 132))),
            "py" => Some(("Python", (53, 114, 165))),
            "js" | "mjs" | "cjs" => Some(("JavaScript", (241, 224, 90))),
            "ts" | "mts" | "cts" => Some(("TypeScript", (49, 120, 198))),
            "jsx" => Some(("JSX", (141, 199, 243))),
            "tsx" => Some(("TSX", (49, 120, 198))),
            "go" => Some(("Go", (0, 173, 216))),
            "rb" => Some(("Ruby", (112, 21, 22))),
            "c" | "h" => Some(("C", (85, 85, 85))),
            "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" => Some(("C++", (243, 75, 125))),
            "cs" => Some(("C#", (23, 134, 0))),
            "java" | "jsp" => Some(("Java", (176, 114, 25))),
            "sh" | "bash" | "zsh" | "fish" | "ksh" => Some(("Shell", (137, 224, 81))),
            "html" | "htm" => Some(("HTML", (227, 76, 38))),
            "css" => Some(("CSS", (86, 61, 124))),
            "scss" | "sass" => Some(("SCSS", (198, 83, 140))),
            "less" => Some(("Less", (29, 54, 93))),
            "dart" => Some(("Dart", (0, 180, 171))),
            "kt" | "kts" => Some(("Kotlin", (169, 123, 255))),
            "swift" => Some(("Swift", (240, 81, 56))),
            "zig" => Some(("Zig", (236, 130, 21))),
            "lua" => Some(("Lua", (0, 0, 128))),
            "hs" => Some(("Haskell", (94, 80, 134))),
            "scala" => Some(("Scala", (194, 45, 64))),
            "ex" | "exs" => Some(("Elixir", (78, 47, 99))),
            "clj" | "cljs" | "cljc" | "edn" => Some(("Clojure", (219, 88, 55))),
            "pl" | "pm" => Some(("Perl", (2, 152, 130))),
            "r" | "R" | "rda" => Some(("R", (25, 145, 208))),
            "m" | "mm" => Some(("Objective-C", (67, 142, 255))),
            "vim" => Some(("Vim script", (19, 191, 123))),
            "php" => Some(("PHP", (119, 123, 180))),
            "svelte" => Some(("Svelte", (255, 62, 0))),
            "vue" => Some(("Vue", (65, 184, 131))),
            "astro" => Some(("Astro", (255, 90, 3))),
            "asm" | "s" | "S" => Some(("Assembly", (110, 76, 19))),
            "erl" | "hrl" => Some(("Erlang", (184, 57, 152))),
            "cr" => Some(("Crystal", (0, 1, 0))),
            "nim" | "nims" => Some(("Nim", (55, 119, 91))),
            "ml" | "mli" => Some(("OCaml", (59, 225, 51))),
            "fs" | "fsx" | "fsscript" => Some(("F#", (184, 69, 252))),
            "f90" | "f95" | "f03" | "f08" | "f" | "for" => Some(("Fortran", (77, 65, 177))),
            "adb" | "ads" => Some(("Ada", (2, 248, 140))),
            "pp" | "pas" => Some(("Pascal", (227, 241, 113))),
            "rkt" | "rktd" | "rktl" => Some(("Racket", (60, 92, 170))),
            "ps1" | "psm1" | "psd1" => Some(("PowerShell", (1, 36, 86))),
            "bat" | "cmd" => Some(("Batch", (193, 241, 46))),
            "tex" | "sty" | "cls" => Some(("LaTeX", (61, 97, 23))),
            "nix" => Some(("Nix", (126, 186, 228))),
            "gleam" => Some(("Gleam", (255, 175, 0))),
            "proto" => Some(("Protocol Buffers", (123, 66, 188))),
            "graphql" | "gql" => Some(("GraphQL", (225, 0, 152))),
            "gradle" => Some(("Gradle", (3, 175, 101))),
            "tf" => Some(("Terraform", (91, 51, 255))),
            "lol" => Some(("LOLCODE", (204, 153, 0))),
            "cpy" | "cob" | "cbl" => Some(("COBOL", (0, 0, 0))),
            _ => None,
        }
    } else {
        match path.file_name().and_then(|n| n.to_str()) {
            Some("Makefile" | "makefile" | "GNUmakefile") => {
                Some(("Makefile", (66, 120, 25)))
            }
            Some("Dockerfile" | "dockerfile") => Some(("Dockerfile", (56, 77, 84))),
            Some("CMakeLists.txt") => Some(("CMake", (218, 52, 52))),
            _ => None,
        }
    }
}
