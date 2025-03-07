use std::{error::Error, ptr::NonNull};

use rdif_intc::IrqConfig;
use rdrive::{
    probe::HardwareKind,
    register::{DriverKind, FdtInfo, ProbeKind},
};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let fdt = include_bytes!("../../../data/qemu.dtb");

    rdrive::init(rdrive::DriverInfoKind::Fdt {
        addr: NonNull::new(fdt.as_ptr() as usize as _).unwrap(),
    });
    let register = rdrive::DriverRegister {
        name: "IrqText",
        kind: DriverKind::Intc,
        probe_kinds: &[ProbeKind::Fdt {
            compatibles: &["arm,cortex-a15-gic"],
            on_probe: probe_intc,
        }],
    };

    rdrive::register_add(register);
    rdrive::probe_with_kind(DriverKind::Intc).unwrap();
}

struct IrqTest {}

impl rdif_intc::DriverGeneric for IrqTest {
    fn open(&mut self) -> rdrive::DriverResult {
        todo!()
    }

    fn close(&mut self) -> rdrive::DriverResult {
        todo!()
    }
}

impl rdif_intc::Interface for IrqTest {
    fn current_cpu_setup(&self) -> rdif_intc::HardwareCPU {
        todo!()
    }

    fn irq_enable(&mut self, _irq: rdrive::IrqId) {
        todo!()
    }

    fn irq_disable(&mut self, _irq: rdrive::IrqId) {
        todo!()
    }

    fn set_priority(&mut self, _irq: rdrive::IrqId, _priority: usize) {
        todo!()
    }

    fn set_trigger(&mut self, _irq: rdrive::IrqId, _trigger: rdif_intc::Trigger) {
        todo!()
    }

    fn set_target_cpu(&mut self, _irq: rdrive::IrqId, _cpu: rdif_intc::CpuId) {
        todo!()
    }

    fn capabilities(&self) -> Vec<rdif_intc::Capability> {
        vec![rdif_intc::Capability::FdtParseConfigFn(parser)]
    }
}

fn parser(_prop_interrupts_one_cell: &[u32]) -> Result<IrqConfig, Box<dyn Error>> {
    Ok(IrqConfig {
        irq: 0.into(),
        trigger: rdif_intc::Trigger::EdgeBoth,
    })
}

fn probe_intc(_info: FdtInfo) -> Result<Vec<HardwareKind>, Box<dyn Error>> {
    Ok(vec![HardwareKind::Intc(Box::new(IrqTest {}))])
}
