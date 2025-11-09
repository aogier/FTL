use crate::api::stats::OvertimeSlot;
use crate::raw;
use crate::shmem::reader::ShmemReader;
use std::time::{SystemTime, UNIX_EPOCH};

impl ShmemReader {
    /// Ottieni dati overtime (time series degli ultimi N slot da 10 minuti)
    pub fn overtime_data(&self) -> Vec<OvertimeSlot> {
        let overtime = self.overtime_array();
        overtime
            .iter()
            .filter(|slot| slot.timestamp > 0) // Filtra slot non inizializzati
            .map(|raw_slot| self.convert_overtime_slot(raw_slot))
            .collect()
    }

    /// Converti raw overTimeData a OvertimeSlot
    fn convert_overtime_slot(&self, raw_slot: &raw::overTimeData) -> OvertimeSlot {
        // Converti timestamp
        let timestamp = UNIX_EPOCH + std::time::Duration::from_secs(raw_slot.timestamp as u64);

        // Totali (convertiti da i32 a u32)
        let total = raw_slot.total.max(0) as u32;
        let blocked = raw_slot.blocked.max(0) as u32;
        let cached = raw_slot.cached.max(0) as u32;

        // Forwarded = total - blocked - cached
        let forwarded = total.saturating_sub(blocked).saturating_sub(cached);

        OvertimeSlot {
            timestamp,
            total,
            blocked,
            cached,
            forwarded,
        }
    }
}
