use super::DailyClock;
use crate::{town_plans::TownPlanBank, Sim};
use p3_core::{
    ai::settlement::SettlementLocation,
    route::{
        finder::RoutePlanner,
        planner::{NavData, NavGrid, TownSlot},
    },
    tick::TickSystems,
};

#[test]
fn assigned_founding_resumes_scheduler_and_survives_save() {
    let (Ok(root), Ok(cpr)) = (std::env::var("P3_SAVES"), std::env::var("P3_CPR")) else {
        return;
    };
    let bytes = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
    let mut sim = Sim::from_save_bytes(&bytes).unwrap();
    let mut model = p3_save::model::SaveModel::parse(&bytes).unwrap();
    let n = model.world.towns.len();
    let used = model.world.header[0x20..0x20 + n].to_vec();
    let id = (0..40u8).find(|x| !used.contains(x)).unwrap();
    let bank = TownPlanBank::from_archives(&[&std::fs::read(cpr).unwrap()]).unwrap();
    let mut locations = [SettlementLocation { x: 0, y: 0, kind: 0 }; 40];
    let mut slots = vec![];
    for i in 0..40 {
        let x = 400 + (i % 8) * 25;
        let y = 400 + (i / 8) * 25;
        locations[i as usize].x = x;
        locations[i as usize].y = y;
        slots.push(TownSlot {
            x: x - 10,
            y: y - 10,
            anchor_x: x,
            anchor_y: y,
        });
    }
    let nav = NavData {
        grid: NavGrid {
            cells: vec![0; 640 * 472],
            width: 640,
            height: 472,
            scale: 3536,
        },
        nodes: vec![(140, 140)],
        node_count: 1,
        matrix: vec![vec![(0, 0)]],
    };
    let mut order = used.clone();
    order.push(id);
    let mut planner = RoutePlanner::new(nav, slots, order, n as u32, vec![], vec![]);
    assert!(planner.build_town_route_table(100));
    let block = model.ships.write();
    let snapshot = p3_save::ships::Ships::parse(&block)
        .unwrap()
        .navigation_snapshot()
        .unwrap();
    let ports = used
        .iter()
        .map(|&i| (locations[usize::from(i)].x as i16, locations[usize::from(i)].y as i16))
        .collect::<Vec<_>>();
    sim.host.navigation = Some(crate::navigation::LiveVoyage::idle(snapshot, ports.clone(), 0));
    sim.host.settlement_locations = Some(locations);
    sim.host.settlement_inland = Some([0; 40]);
    sim.host.settlement_plans = Some(bank);
    sim.host.navigation_planner = Some(planner);
    sim.host.foundation_names = Some((75, 80));
    sim.host.expedition_resources = Some(crate::HanseFleetResources {
        ship_names: vec![],
        first_names: 75,
        last_names: 80,
        ports: ports.iter().map(|&(x, y)| Some((x as u16, y as u16))).collect(),
        hotseat: false,
        player: 0,
    });
    let tick = sim.idle.clock.tick;
    let mut scheduler = sim.host.council_scheduler.clone();
    let mut at = scheduler.head;
    while at < scheduler.capacity {
        let record = &mut scheduler.records[at as usize];
        record.due = tick + 0x100000;
        at = record.next;
    }
    while {
        let mut n = 0;
        let mut free = scheduler.free_head;
        while free < scheduler.capacity && n < 4 {
            n += 1;
            free = scheduler.records[free as usize].next;
        }
        n < 4
    } {
        let id = scheduler.add_task(tick + 0x100000);
        scheduler.records[id as usize].kind = 0x37;
    }
    let mission = scheduler.add_task(tick);
    let task = &mut scheduler.records[mission as usize];
    task.kind = 0x32;
    task.set_u8_at(0xc, 0);
    task.set_u8_at(0xd, 0);
    task.set_u8_at(0xf, 0);
    task.set_u8_at(0x14, id);
    task.set_u8_at(0x15, 0);
    let later = scheduler.add_task(tick);
    let task = &mut scheduler.records[later as usize];
    task.kind = 0x37;
    task.set_u32_at(8, 0);
    task.set_u32_at(0xc, 0);
    sim.rep_merchants[0].set_i32(0x608, 1);
    sim.economy.merchants[0].set_i32(0x608, 1);
    sim.host.council_scheduler = scheduler;
    let original_scheduler = sim.host.council_scheduler.clone();
    let expected = original_scheduler.records[mission as usize];
    let free = original_scheduler.free_head;
    sim.host.council_scheduler.records[free as usize].next = free;
    assert!(sim
        .start_assigned_founding_mission(mission, expected)
        .unwrap_err()
        .contains("cíclica"));
    assert_eq!(sim.economy.world.towns.len(), n);
    sim.host.council_scheduler = original_scheduler.clone();
    sim.host.council_scheduler.free_head = sim.host.council_scheduler.capacity;
    assert!(sim
        .start_assigned_founding_mission(mission, expected)
        .unwrap_err()
        .contains("crecimiento"));
    assert_eq!(sim.economy.world.towns.len(), n);
    sim.host.council_scheduler = original_scheduler.clone();
    sim.host.council_scheduler.records[free as usize].next = mission;
    assert!(sim
        .start_assigned_founding_mission(mission, expected)
        .unwrap_err()
        .contains("ocupada"));
    assert_eq!(sim.economy.world.towns.len(), n);
    sim.host.council_scheduler = original_scheduler;
    {
        let mut clock = DailyClock {
            sim: &mut sim,
            world_ready: false,
            error: None,
        };
        clock.scheduled_tasks_tick(tick);
        assert!(clock.error.is_none(), "{:?}", clock.error);
    }
    assert!(sim.host.pending_founding.is_none());
    assert_eq!(sim.economy.world.towns.len(), n + 1);
    assert_eq!(sim.idle.clock.town_count as usize, n + 1);
    assert_eq!(sim.economy.world.towns[n].u8(0x6f1), 0);
    assert_eq!(sim.rep_merchants[0].i32(0x608) & 1, 0);
    let task = sim.host.council_scheduler.records[mission as usize];
    assert_eq!(task.u8_at(0xf), 1);
    assert_eq!(task.u8_at(0x16), n as u8);
    assert_eq!(task.due, tick + 0x100);
    assert!(sim
        .host
        .council_scheduler
        .records
        .iter()
        .any(|r| r.kind == 0x39 && r.u8_at(8) == n as u8));
    sim.persist_all(&mut model).unwrap();
    let saved = model.write();
    let reloaded = Sim::from_save_bytes(&saved).unwrap();
    assert_eq!(reloaded.economy.world.towns.len(), n + 1);
    assert_eq!(reloaded.host.council_scheduler.records[mission as usize], task);
    assert_eq!(reloaded.economy.world.towns[n].u8(0x6f1), 0);
}
