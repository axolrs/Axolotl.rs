// Owner: PascalElixir / axolrs (GitHub org)
// File: language server command resolution with PATH lookup and command fallback.

import * as fs from 'node:fs';
import * as path from 'node:path';

export const DEFAULT_SERVER_PATH = 'axol-analyzer';
export const DEFAULT_SERVER_FALLBACK = 'bucket gills --stdio';

export interface ServerExecutable {
  readonly command: string;
  readonly args: readonly string[];
}

const WINDOWS_PATH_EXT_DEFAULT = '.COM;.EXE;.BAT;.CMD';

/** Splits a command line into tokens, honoring double and single quoted segments. */
export function splitCommandLine(line: string): string[] {
  const tokens: string[] = [];
  let current = '';
  let started = false;
  let quote: '"' | "'" | undefined = undefined;
  for (let index = 0; index < line.length; index += 1) {
    const ch = line[index];
    if (quote === '"') {
      if (ch === '\\') {
        const next = line[index + 1];
        if (next === '"' || next === '\\' || next === '$') {
          current += next;
          index += 1;
        } else {
          current += ch;
        }
      } else if (ch === '"') {
        quote = undefined;
      } else {
        current += ch;
      }
    } else if (quote === "'") {
      if (ch === "'") {
        quote = undefined;
      } else {
        current += ch;
      }
    } else if (ch === '"' || ch === "'") {
      quote = ch;
      started = true;
    } else if (ch === ' ' || ch === '\t') {
      if (started) {
        tokens.push(current);
        current = '';
        started = false;
      }
    } else {
      current += ch;
      started = true;
    }
  }
  if (started) {
    tokens.push(current);
  }
  return tokens;
}

/** Reports whether a path refers to an existing executable file. */
function isExecutableFile(candidatePath: string): boolean {
  try {
    const stats = fs.statSync(candidatePath);
    if (!stats.isFile()) {
      return false;
    }
    fs.accessSync(candidatePath, fs.constants.X_OK);
    return true;
  } catch {
    return false;
  }
}

/** Returns the candidate Windows executable extensions derived from PATHEXT. */
function windowsExtensions(env: NodeJS.ProcessEnv): string[] {
  const pathExt = env.PATHEXT ?? WINDOWS_PATH_EXT_DEFAULT;
  const extensions = pathExt
    .split(';')
    .map((value) => value.trim())
    .filter((value) => value.length > 0);
  return ['', ...extensions];
}

/** Resolves a command name to an executable path using the given PATH environment. */
export function resolveExecutable(
  command: string,
  env: NodeJS.ProcessEnv,
  platform: NodeJS.Platform,
): string | undefined {
  if (command.length === 0) {
    return undefined;
  }
  if (command.includes('/') || command.includes('\\')) {
    return isExecutableFile(command) ? command : undefined;
  }
  const pathVariable = env.PATH ?? env.Path;
  if (pathVariable === undefined) {
    return undefined;
  }
  const separator = platform === 'win32' ? ';' : ':';
  const directories = pathVariable.split(separator).filter((dir) => dir.length > 0);
  const extensions = platform === 'win32' ? windowsExtensions(env) : [''];
  for (const directory of directories) {
    for (const extension of extensions) {
      const candidate = path.join(directory, `${command}${extension}`);
      if (isExecutableFile(candidate)) {
        return candidate;
      }
    }
  }
  return undefined;
}

/** Picks the primary server command when it is on PATH, otherwise the fallback command. */
export function resolveServerOptions(
  primary: string,
  fallback: string,
  env: NodeJS.ProcessEnv,
  platform: NodeJS.Platform,
): ServerExecutable {
  const primaryTokens = splitCommandLine(primary.trim());
  const primaryResolved = primaryTokens.length > 0 ? resolveExecutable(primaryTokens[0], env, platform) : undefined;
  if (primaryResolved !== undefined) {
    return { command: primaryResolved, args: primaryTokens.slice(1) };
  }
  const fallbackTokens = splitCommandLine(fallback.trim());
  const fallbackResolved = fallbackTokens.length > 0 ? resolveExecutable(fallbackTokens[0], env, platform) : undefined;
  if (fallbackResolved !== undefined) {
    return { command: fallbackResolved, args: fallbackTokens.slice(1) };
  }
  return {
    command: primaryTokens.length > 0 ? primaryTokens[0] : DEFAULT_SERVER_PATH,
    args: primaryTokens.slice(1),
  };
}
