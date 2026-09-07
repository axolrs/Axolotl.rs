/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Site-wide constants - nav links, project metadata, owner handles, and license summary.
*/

export const siteConfig = {
  name: 'Axolotl',
  shortName: 'Axolotl',
  tagline:
    'A high-level, all-purpose, memory-safe, zero-GC programming language with the smartest compiler you have ever used.',
  blurb:
    'Lua-like surface. Rust spine. Compiles to idiomatic Rust and ships as a native binary - no runtime, no VM, no GC, ever.',
  url: 'https://axolotl.rs',
  repo: 'https://github.com/axolrs/axolotl',
  docsRepo: 'https://github.com/axolrs/axolotl/tree/main/axolotl-docs',
  org: 'axolrs',
  owner: 'PascalElixir',
  year: 2026,
  license: 'MIT OR Apache-2.0',
  install: 'curl -sSf https://axolotl.rs/install | sh',
};

export interface NavLink {
  href: string;
  label: string;
  description?: string;
}

export const mainNav: NavLink[] = [
  { href: '/', label: 'Home' },
  { href: '/docs', label: 'Docs', description: 'Design documents, the language bible, and reference material.' },
  { href: '/learn', label: 'Learn', description: 'A chaptered tour of the language from hello-world to FFI.' },
  { href: '/benchmarks', label: 'Benchmarks', description: 'The 10 build-your-own-x ports that prove zero overhead.' },
  { href: '/play', label: 'Play', description: 'In-browser playground. Coming soon.' },
  { href: '/about', label: 'About', description: 'License, owners, and where to find the project.' },
];

export const socialLinks: NavLink[] = [
  { href: 'https://github.com/axolrs/axolotl', label: 'GitHub' },
  { href: 'https://github.com/axolrs/axolotl/issues', label: 'Issues' },
  { href: 'https://github.com/axolrs/axolotl/discussions', label: 'Discussions' },
];
