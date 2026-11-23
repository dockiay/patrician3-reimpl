//! Programación anual de la elección de alcalde (tarea 0x02).
use p3_core::{calendar::Calendar, scheduler::TaskRecord};

/// Calcula la siguiente fecha y señala si debe celebrarse la elección.
/// Los ticks altos con fecha sin inicializar aún no están acreditados aquí.
pub(crate) fn schedule(task: &mut TaskRecord, year: u16, day_of_year: u16) -> Result<Option<u8>, String> {
    if task.kind != 2 {
        return Err("tarea de elección inválida".into());
    }
    let mut next = *task;
    let mut day = u16::from_le_bytes([next.u8_at(8), next.u8_at(9)]);
    if day > 364 {
        if next.due > i32::MAX as u32 {
            return Err("fecha inicial de elección fuera del rango acreditado".into());
        }
        day = ((next.due >> 8) % 365) as u16;
        let date = Calendar::for_tick(next.due);
        next.set_u8_at(8, day as u8);
        next.set_u8_at(9, (day >> 8) as u8);
        next.set_u8_at(0xf, date.day);
        next.set_u8_at(0x10, date.month + 1);
    }
    let election = (day == day_of_year).then_some(next.u8_at(0xe));
    let target = u32::from(year) + u32::from(day <= day_of_year);
    next.due = target.wrapping_mul(365).wrapping_add(u32::from(day)).wrapping_mul(256) | 0xfa;
    *task = next;
    Ok(election)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    #[test]
    fn scheduling_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let mut cases = Vec::new();
        let mut actual = Vec::new();
        for i in 0..512u32 {
            let mut task = TaskRecord {
                due: (i * 817319) & 0x7fffffff,
                next: 0xffff,
                kind: 2,
                ..Default::default()
            };
            let date = if i % 4 == 0 { u16::MAX } else { (i % 365) as u16 };
            task.set_u8_at(8, date as u8);
            task.set_u8_at(9, (date >> 8) as u8);
            task.set_u8_at(0xe, (i % 30) as u8);
            task.set_u8_at(0xf, 17);
            task.set_u8_at(0x10, 11);
            let year = (i * 37) as u16;
            let day = match i % 3 {
                0 => (i % 365) as u16,
                1 => 0,
                _ => 364,
            };
            let encode = |t: &TaskRecord| {
                [
                    t.due.to_le_bytes().as_slice(),
                    t.next.to_le_bytes().as_slice(),
                    t.kind.to_le_bytes().as_slice(),
                    t.payload.as_slice(),
                ]
                .concat()
            };
            cases.push(serde_json::json!({"task":hex(&encode(&task)),"year":year,"day_of_year":day}));
            let called = schedule(&mut task, year, day).unwrap().into_iter().collect::<Vec<_>>();
            actual.push(serde_json::json!({"task":hex(&encode(&task)),"called":called}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_mayor_schedule.py");
        let mut child = std::process::Command::new(python)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let expected: Vec<serde_json::Value> = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(actual, expected);
    }
}
