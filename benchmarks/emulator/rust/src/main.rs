// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/emulator/rust/src/main.rs - CHIP-8 full opcode set with 500k cycles.
//
// Reference. Mirrors ../axol/src/main.axol exactly. Uses the same `handled` flag
// dispatch pattern as the axol port (avoids if/else chain differences).

use std::env;

const DEFAULT_WORKLOAD: i64 = 5000;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let font: [i64; 16] = [240, 144, 144, 144, 240, 32, 96, 32, 32, 112, 240, 16, 240, 128, 240, 240];
    let mut mem: Vec<i64> = Vec::new();
    let mut k: i64 = 0;
    while k < 4096 {
        mem.push(if k < 16 { font[k as usize] } else { 0 });
        k += 1;
    }
    let mut v: Vec<i64> = vec![0; 16];
    let mut i_reg: i64 = 0;
    let mut pc: i64 = 0;
    let mut cycles: i64 = 0;
    let mut stack: Vec<i64> = Vec::new();
    let mut j: i64 = 0;
    while j < workload {
        let mut op: i64 = 4660;
        let m = j % 16;
        if m == 0 { op = 24832 | (j & 255); }
        if m == 1 { op = 29184 | (j & 255); }
        if m == 2 { op = 33040; }
        if m == 3 { op = 33043; }
        if m == 4 { op = 33044; }
        if m == 5 { op = 12289; }
        if m == 6 { op = 16386; }
        if m == 7 { op = 40960 | (j & 4095); }
        if m == 8 { op = 53269; }
        if m == 9 { op = 28673; }
        if m == 10 { op = 32772; }
        if m == 11 { op = 12291; }
        if m == 12 { op = 61461; }
        if m == 13 { op = 61507; }
        if m == 14 { op = 238; }
        let nib0 = (op / 4096) & 15;
        let nib1 = (op / 256) & 15;
        let nib2 = (op / 16) & 15;
        let nib3 = op & 15;
        let nnn = op & 4095;
        let nn = op & 255;
        let x = nib1;
        let y = nib2;
        let mut handled: i64 = 0;
        if op == 238 {
            if !stack.is_empty() {
                let sp = stack.len() as i64;
                let idx = sp - 1;
                pc = stack[idx as usize];
            }
            handled = 1;
        }
        if handled == 0 {
            if nib0 == 1 {
                pc = nnn;
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 2 {
                stack.push(pc);
                pc = nnn;
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 3 {
                if v[x as usize] == nn { pc += 2; }
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 4 {
                if v[x as usize] != nn { pc += 2; }
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 6 {
                v[x as usize] = nn;
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 7 {
                v[x as usize] = (v[x as usize] + nn) & 255;
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 8 {
                if nib3 == 0 {
                    v[x as usize] = v[y as usize];
                }
                if nib3 == 1 {
                    v[x as usize] = (v[x as usize] | v[y as usize]) & 255;
                }
                if nib3 == 2 {
                    v[x as usize] = (v[x as usize] & v[y as usize]) & 255;
                }
                if nib3 == 4 {
                    let s = v[x as usize] + v[y as usize];
                    if s > 255 { v[15] = 1; } else { v[15] = 0; }
                    v[x as usize] = s & 255;
                }
                if nib3 == 6 {
                    let b = v[x as usize] & 1;
                    v[x as usize] = (v[x as usize] / 2) & 255;
                    v[15] = b;
                }
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 10 {
                i_reg = nnn;
                handled = 1;
            }
        }
        if handled == 0 {
            if nib0 == 15 {
                if nn == 85 {
                    let mut kk: i64 = 0;
                    while kk <= x {
                        mem[(i_reg + kk) as usize] = v[kk as usize];
                        kk += 1;
                    }
                }
                if nn == 101 {
                    let mut kk: i64 = 0;
                    while kk <= x {
                        v[kk as usize] = mem[(i_reg + kk) as usize];
                        kk += 1;
                    }
                }
                handled = 1;
            }
        }
        cycles += 1;
        j += 1;
    }
    let mut csum: i64 = 0;
    let mut k: i64 = 0;
    while k < 16 {
        csum = csum.wrapping_mul(31).wrapping_add(v[k as usize]);
        k += 1;
    }
    println!("{} {} {} {} {} {} {} {}", "emulator: cycles=", cycles, "v0=", v[0], "i=", i_reg, "checksum=", csum);
}
