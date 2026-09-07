// Owner: PascalElixir / axolrs (GitHub org)
// File: benchmarks/cron/rust/src/main.rs - 5-field cron scheduler with bitmask lookup.
//
// Reference. Mirrors ../axol/src/main.axol exactly.

const DEFAULT_WORKLOAD: i64 = 20_000_000;

fn main() {
    let workload: i64 = DEFAULT_WORKLOAD;
    let mut pow2: Vec<i64> = vec![];
    let mut p: i64 = 1;
    let mut i: i64 = 0;
    while i < 64 { pow2.push(p); p = p * 2; i = i + 1; }
    let mut mask_min: i64 = 0;
    let mut mask_hour: i64 = 0;
    let mut mask_day: i64 = 0;
    let mut mask_month: i64 = 0;
    let mut mask_dow: i64 = 0;
    i = 0;
    while i < 60 { mask_min |= pow2[i as usize]; i = i + 1; }
    i = 0;
    while i < 24 { mask_hour |= pow2[i as usize]; i = i + 1; }
    i = 0;
    while i < 32 { mask_day |= pow2[i as usize]; i = i + 1; }
    i = 0;
    while i < 13 { mask_month |= pow2[i as usize]; i = i + 1; }
    i = 0;
    while i < 7 { mask_dow |= pow2[i as usize]; i = i + 1; }
    let m_one: i64 = 1;
    let e0_min = mask_min;  let e0_hour = mask_hour; let e0_day = mask_day;   let e0_month = mask_month; let e0_dow = mask_dow;
    let e1_min = m_one;     let e1_hour = mask_hour; let e1_day = mask_day;   let e1_month = mask_month; let e1_dow = mask_dow;
    let e2_min = m_one;     let e2_hour = m_one;     let e2_day = mask_day;   let e2_month = mask_month; let e2_dow = mask_dow;
    let e3_min = m_one;     let e3_hour = m_one;     let e3_day = m_one;      let e3_month = mask_month; let e3_dow = mask_dow;
    let e4_min = m_one;     let e4_hour = m_one;     let e4_day = mask_day;   let e4_month = mask_month; let e4_dow = m_one;
    let mut fired: i64 = 0;
    i = 0;
    while i < workload {
        let minute = i % 60;
        let hour = (i / 60) % 24;
        let day = (i / 1440) % 28 + 1;
        let month = (i / 40320) % 12 + 1;
        let dow = (i / 1440) % 7;
        let pm = pow2[minute as usize];
        let ph = pow2[hour as usize];
        let pd = pow2[day as usize];
        let pmo = pow2[month as usize];
        let pw = pow2[dow as usize];
        if (e0_min & pm) != 0 && (e0_hour & ph) != 0 && (e0_day & pd) != 0 && (e0_month & pmo) != 0 && (e0_dow & pw) != 0 { fired += 1; }
        if (e1_min & pm) != 0 && (e1_hour & ph) != 0 && (e1_day & pd) != 0 && (e1_month & pmo) != 0 && (e1_dow & pw) != 0 { fired += 1; }
        if (e2_min & pm) != 0 && (e2_hour & ph) != 0 && (e2_day & pd) != 0 && (e2_month & pmo) != 0 && (e2_dow & pw) != 0 { fired += 1; }
        if (e3_min & pm) != 0 && (e3_hour & ph) != 0 && (e3_day & pd) != 0 && (e3_month & pmo) != 0 && (e3_dow & pw) != 0 { fired += 1; }
        if (e4_min & pm) != 0 && (e4_hour & ph) != 0 && (e4_day & pd) != 0 && (e4_month & pmo) != 0 && (e4_dow & pw) != 0 { fired += 1; }
        i = i + 1;
    }
    println!("{} {} {} {} {} {}", "cron: ticks=", workload, "entries=", 5, "fired=", fired);
}
