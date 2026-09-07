/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Loader for /benchmarks - exposes the 10 build-your-own-x ports used to prove zero overhead vs. hand-written Rust.
*/

import { benchmarkProjects } from '$lib/benchmarks';

export const prerender = true;

export function load() {
  return {
    projects: benchmarkProjects,
    total: benchmarkProjects.length,
  };
}
