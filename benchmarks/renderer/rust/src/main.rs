// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/renderer/rust/src/main.rs - software 3D wireframe rasterizer with Bresenham line drawing
//
// Reference implementation. Mirrors ../axol/src/main.axol exactly;
// the bench harness in ../bench/run.sh asserts byte-identical stdout
// and axol_time / rust_time <= 1.05.

use std::env;

const DEFAULT_WORKLOAD: i64 = 500;
const W: i64 = 80;
const H: i64 = 50;

fn main() {
    let workload: i64 = env::args().nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(DEFAULT_WORKLOAD);
    let verts_x: [i64; 8] = [-1000, 1000, 1000, -1000, -1000, 1000, 1000, -1000];
    let verts_y: [i64; 8] = [-1000, -1000, 1000, 1000, -1000, -1000, 1000, 1000];
    let verts_z: [i64; 8] = [-1000, -1000, -1000, -1000, 1000, 1000, 1000, 1000];
    let edges_a: [i64; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3];
    let edges_b: [i64; 12] = [1, 2, 3, 0, 5, 6, 7, 4, 4, 5, 6, 7];
    let mut buf: Vec<i64> = vec![0; (W * H) as usize];
    let mut drawn: i64 = 0;
    let mut i: i64 = 0;
    while i < workload {
        let mut k: i64 = 0;
        while k < buf.len() as i64 { buf[k as usize] = 0; k += 1; }
        let angle = i * 10;
        let ca = 1000 - (angle * angle) / 2000000;
        let sa = angle - (angle * angle * angle) / 6000000;
        let mut px: [i64; 8] = [0; 8];
        let mut py: [i64; 8] = [0; 8];
        let mut k: i64 = 0;
        while k < 8 {
            let xr = (verts_x[k as usize] * ca - verts_z[k as usize] * sa) / 1000;
            let zr = (verts_x[k as usize] * sa + verts_z[k as usize] * ca) / 1000 + 5;
            let zc = if zr < 1 { 1 } else { zr };
            px[k as usize] = (xr * (W / 2) / zc) + (W / 2);
            py[k as usize] = (verts_y[k as usize] * (H / 2) / zc) + (H / 2);
            k += 1;
        }
        let mut e: i64 = 0;
        while e < 12 {
            let x0 = px[edges_a[e as usize] as usize];
            let y0 = py[edges_a[e as usize] as usize];
            let x1 = px[edges_b[e as usize] as usize];
            let y1 = py[edges_b[e as usize] as usize];
            let mut dx = (x1 - x0).abs();
            let mut dy = (y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx - dy;
            let mut x = x0;
            let mut y = y0;
            loop {
                if x >= 0 && x < W && y >= 0 && y < H {
                    buf[(y * W + x) as usize] = 1;
                }
                if x == x1 && y == y1 { break; }
                let e2 = 2 * err;
                if e2 > -dy { err -= dy; x += sx; }
                if e2 < dx { err += dx; y += sy; }
                let _ = (&mut dx, &mut dy);
            }
            drawn += 1;
            e += 1;
        }
        i += 1;
    }
    let mut sum: i64 = 0;
    let mut k: i64 = 0;
    while k < buf.len() as i64 { sum += buf[k as usize]; k += 1; }
    println!("{} {} {} {} {} {}", "renderer: frames=", workload, "edges_drawn=", drawn, "pixels=", sum);
}
