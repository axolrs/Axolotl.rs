/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Metadata for the 10 build-your-own-x ports used to prove Axolotl has no overhead vs. hand-written Rust.
*/

export interface BenchmarkProject {
  id: string;
  name: string;
  tagline: string;
  description: string;
  rustCounterpart: string;
  difficulty: 'Beginner' | 'Intermediate' | 'Advanced' | 'Expert';
  status: 'Planned' | 'In Progress' | 'Shipped';
  axolPath: string;
  rustPath: string;
}

export const benchmarkProjects: BenchmarkProject[] = [
  {
    id: 'git',
    name: 'Git',
    tagline: 'A miniature content-addressable VCS.',
    description:
      'A from-scratch port of the core Git object model - blobs, trees, commits, refs, and a packed object store. Proves that Axolotl can express the kind of pointer-juggling, hash-tree-walking systems code people write in C and Rust every day.',
    rustCounterpart: 'libgit2 + custom plumbing',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/git.axol',
    rustPath: 'benchmarks/git.rs',
  },
  {
    id: 'database',
    name: 'Database',
    tagline: 'A toy KV store with a real B-tree and WAL.',
    description:
      'A persistent key-value store with a B+tree on top of a write-ahead log, page allocator, and MVCC reader. Demonstrates ownership-heavy systems work and zero-overhead FFI to the OS page cache.',
    rustCounterpart: 'sled / fjall patterns',
    difficulty: 'Expert',
    status: 'Planned',
    axolPath: 'benchmarks/database.axol',
    rustPath: 'benchmarks/database.rs',
  },
  {
    id: 'redis',
    name: 'Redis',
    tagline: 'An in-memory server speaking RESP.',
    description:
      'A RESP3 server with key expiry, pub/sub, and a single-threaded event loop. Demonstrates async + channels and how cheap they are in Axolotl compared to hand-written Tokio.',
    rustCounterpart: 'tokio + bytes',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/redis.axol',
    rustPath: 'benchmarks/redis.rs',
  },
  {
    id: 'shell',
    name: 'Shell',
    tagline: 'A POSIX-ish shell with job control.',
    description:
      'A real shell: pipes, redirects, job control, signals, and a small bytecode for the interpreter. Proves that systems programming in Axolotl reads like Lua but performs like Rust.',
    rustCounterpart: 'nushelleq / ion',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/shell.axol',
    rustPath: 'benchmarks/shell.rs',
  },
  {
    id: 'web-server',
    name: 'Web Server',
    tagline: 'A from-scratch HTTP/1.1 + HTTP/2 server.',
    description:
      'A multithreaded HTTP/1.1 server with keep-alive, chunked encoding, and a static file cache, plus an HTTP/2 frame layer. Same Axolotl program, two protocols - same performance as the Rust twin.',
    rustCounterpart: 'hyper + tower',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/web-server.axol',
    rustPath: 'benchmarks/web-server.rs',
  },
  {
    id: 'text-editor',
    name: 'Text Editor',
    tagline: 'A modal editor with piece-table editing.',
    description:
      "A piece-table editor with syntax highlighting (tree-sitter via FFI), incremental redraw, and modal keybindings. Exercises Axolotl's ownership, generics, and FFI to a C library.",
    rustCounterpart: 'helix-core',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/text-editor.axol',
    rustPath: 'benchmarks/text-editor.rs',
  },
  {
    id: 'programming-language',
    name: 'Programming Language',
    tagline: 'A tiny compiled language in 1k lines.',
    description:
      'A small lex/parse/codegen pipeline that compiles a Lisp-flavored language to LLVM IR. Proves that Axolotl is good for writing compilers in - the most demanding kind of program.',
    rustCounterpart: 'inkwell + logos + chumsky',
    difficulty: 'Expert',
    status: 'Planned',
    axolPath: 'benchmarks/programming-language.axol',
    rustPath: 'benchmarks/programming-language.rs',
  },
  {
    id: 'regex-engine',
    name: 'Regex Engine',
    tagline: 'A backtracking + NFA regex engine.',
    description:
      'A regex engine with character classes, anchors, captures, lookaround, and a Pike VM for the NFA path. Demonstrates Axolotl enums, pattern matching, and zero-cost iterator chains.',
    rustCounterpart: 'regex / fancy-regex',
    difficulty: 'Advanced',
    status: 'Planned',
    axolPath: 'benchmarks/regex-engine.axol',
    rustPath: 'benchmarks/regex-engine.rs',
  },
  {
    id: 'docker',
    name: 'Docker',
    tagline: 'A minimal container runtime.',
    description:
      'A container runtime on Linux: namespaces, cgroups, layered overlayfs, and an OCI image unpacker. Exercises raw syscalls, FFI, and unsafe - all from Axolotl, all with the same perf as Rust.',
    rustCounterpart: 'youki / containers/oci-spec',
    difficulty: 'Expert',
    status: 'Planned',
    axolPath: 'benchmarks/docker.axol',
    rustPath: 'benchmarks/docker.rs',
  },
  {
    id: 'emulator-vm',
    name: 'Emulator / VM',
    tagline: 'A RISC-V emulator with a JIT.',
    description:
      'A RISC-V RV32I emulator with a basic block-copying JIT. Demonstrates that Axolotl can do bit-level systems work, raw memory, and unsafe pointer manipulation exactly like Rust - with no extra cost.',
    rustCounterpart: 'riscv-rust + cranelift',
    difficulty: 'Expert',
    status: 'Planned',
    axolPath: 'benchmarks/emulator-vm.axol',
    rustPath: 'benchmarks/emulator-vm.rs',
  },
];
