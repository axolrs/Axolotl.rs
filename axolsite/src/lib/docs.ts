/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Shared docs metadata, slug helpers, and TOC extractor for the axolsite documentation routes.
*/

export interface DocMeta {
  slug: string;
  title: string;
  description: string;
  category: DocCategory;
  filename: string;
}

export type DocCategory = 'Guide' | 'Language' | 'Design' | 'Toolchain';

export interface TocItem {
  depth: number;
  text: string;
  id: string;
}

export const docsList: DocMeta[] = [
  {
    slug: 'getting-started',
    title: 'Getting Started',
    description: 'Install the toolchain, create your first project, run it in interpreted and compiled modes.',
    category: 'Guide',
    filename: 'getting-started.md',
  },
  {
    slug: 'language-reference',
    title: 'Language Reference',
    description: 'The complete Axolotl (.axol) language reference: every construct, every type, every operator.',
    category: 'Language',
    filename: 'language-reference.md',
  },
  {
    slug: 'ownership',
    title: 'Ownership',
    description: "How Axolotl's ownership system maps to Rust's: borrow, mut, move, and the borrow checker.",
    category: 'Language',
    filename: 'ownership.md',
  },
  {
    slug: 'high-level',
    title: 'High-Level Surface',
    description: 'The enjoyable ergonomics: match replacements, let-else, pipe, named args, comprehensions.',
    category: 'Language',
    filename: 'high-level.md',
  },
  {
    slug: 'toolchain',
    title: 'Toolchain',
    description: 'The bucket build orchestrator, axolc compiler, axol-hot-runner interpreter, Gills LSP.',
    category: 'Toolchain',
    filename: 'toolchain.md',
  },
  {
    slug: 'interpreter',
    title: 'Interpreter + JIT',
    description: 'The axol-hot-runner design for sub-200ms dev iteration. Tree-walk, bytecode, Cranelift JIT.',
    category: 'Toolchain',
    filename: 'interpreter.md',
  },
  {
    slug: 'gills',
    title: 'Gills (LSP)',
    description: 'The rust-analyzer fork that understands both .axol and .rs files.',
    category: 'Toolchain',
    filename: 'gills.md',
  },
  {
    slug: 'architecture',
    title: 'Architecture',
    description: 'The five candidate architectures, the chosen one, and why the others were rejected.',
    category: 'Design',
    filename: 'architecture.md',
  },
  {
    slug: 'comparison',
    title: 'Comparison',
    description: 'Axolotl vs Lua, Rust, TypeScript, Nim, Zig, Mojo, Carbon.',
    category: 'Design',
    filename: 'comparison.md',
  },
  {
    slug: 'examples',
    title: 'Examples',
    description: 'A collection of complete Axolotl examples: hello world, structs, enums, closures, async.',
    category: 'Guide',
    filename: 'examples.md',
  },
  {
    slug: 'editor-setup',
    title: 'Editor Setup',
    description: 'How to set up Zed, Neovim, and VS Code for Axolotl development with Gills (the LSP).',
    category: 'Guide',
    filename: 'editor-setup.md',
  },
  {
    slug: 'faq',
    title: 'FAQ',
    description: 'Frequently asked questions about Axolotl.',
    category: 'Guide',
    filename: 'faq.md',
  },
  {
    slug: 'contributing',
    title: 'Contributing',
    description: 'How to contribute to Axolotl: the workflow rule, the hard rules, the project structure.',
    category: 'Guide',
    filename: 'contributing.md',
  },
];

export const docCategories: DocCategory[] = [
  'Guide',
  'Language',
  'Design',
  'Toolchain',
];

export function docsByCategory(): Record<DocCategory, DocMeta[]> {
  const map: Record<DocCategory, DocMeta[]> = {
    Guide: [],
    Language: [],
    Design: [],
    Toolchain: [],
  };
  for (const d of docsList) map[d.category].push(d);
  return map;
}

export function getDocMeta(slug: string): DocMeta | undefined {
  return docsList.find((d) => d.slug === slug);
}

export function slugify(text: string): string {
  return text
    .replace(/<[^>]*>/g, '')
    .replace(/&[#a-z0-9]+;/gi, '')
    .toLowerCase()
    .replace(/[^\w\s-]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-+|-+$/g, '');
}

export function extractToc(md: string): TocItem[] {
  const lines = md.split('\n');
  const toc: TocItem[] = [];
  let inCodeBlock = false;
  for (const line of lines) {
    if (/^```/.test(line.trim())) {
      inCodeBlock = !inCodeBlock;
      continue;
    }
    if (inCodeBlock) continue;
    const m = /^(#{1,6})\s+(.+?)\s*$/.exec(line);
    if (!m) continue;
    const depth = m[1].length;
    const text = m[2].replace(/[`*_~]/g, '').trim();
    if (!text) continue;
    const escapedForSlug = text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
    toc.push({ depth, text, id: slugify(escapedForSlug) });
  }
  return toc;
}
