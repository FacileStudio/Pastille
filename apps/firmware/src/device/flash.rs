//! Settings in the `pastille` data partition: two 4 KiB slots written alternately, each holding one
//! CRC-checked record, so a power cut during a write keeps the previous record. The partition is
//! flagged `encrypted`, so once flash encryption is enabled in eFuse its contents are encrypted.

use crate::link::{StoreRequest, STORE_CHANNEL};
use crate::store::{decode_record, encode_record, newest, Stored, RECORD_BYTES};
use esp_bootloader_esp_idf::partitions::{read_partition_table, PartitionEntry, PARTITION_TABLE_MAX_LEN};
use esp_hal::peripherals::FLASH;
use esp_storage::FlashStorage;

/// Label of the data partition in `partitions.csv`.
const PARTITION_LABEL: &str = "pastille";
const SLOT_BYTES: u32 = FlashStorage::SECTOR_SIZE;
/// The record rounded up to the 32-byte block encrypted flash writes need.
const IO_BYTES: usize = RECORD_BYTES.next_multiple_of(32);

/// The settings partition and the newest record read from it.
pub struct Settings {
    flash: FlashStorage<'static>,
    partition: PartitionEntry,
    seq: u32,
    pub stored: Stored,
}

impl Settings {
    /// Finds the partition and loads the newest valid record; `None` without the partition.
    pub fn open(flash: FLASH<'static>) -> Option<Self> {
        let mut flash = FlashStorage::new(flash);
        let mut table = [0u8; PARTITION_TABLE_MAX_LEN];
        let entry = read_partition_table(&mut flash, &mut table)
            .ok()?
            .iter()
            .find(|entry| entry.label_as_str() == PARTITION_LABEL && entry.len() >= 2 * SLOT_BYTES)?;
        let mut settings = Self { flash, partition: entry, seq: 0, stored: Stored::default() };
        let latest = newest(settings.read_slot(0), settings.read_slot(1));
        if let Some((seq, stored)) = latest {
            settings.seq = seq;
            settings.stored = stored;
        }
        Some(settings)
    }

    fn read_slot(&mut self, slot: u32) -> Option<(u32, Stored)> {
        let mut buf = [0u8; IO_BYTES];
        self.partition.as_flash_region(&mut self.flash).read(slot * SLOT_BYTES, &mut buf).ok()?;
        decode_record(&buf)
    }

    /// Writes the current settings as the next record, into the slot the previous one is not in.
    fn save(&mut self) -> bool {
        let seq = self.seq.wrapping_add(1);
        let mut record = [0u8; RECORD_BYTES];
        let len = encode_record(seq, &self.stored, &mut record);
        let mut buf = [0xFFu8; IO_BYTES];
        buf[..len].copy_from_slice(&record[..len]);
        let start = (seq % 2) * SLOT_BYTES;
        let mut region = self.partition.as_flash_region(&mut self.flash);
        let written = region.erase(start, start + SLOT_BYTES).is_ok() && region.write(start, &buf).is_ok();
        if written {
            self.seq = seq;
        }
        written
    }
}

/// Persists every config and bond the other tasks hand over.
#[embassy_executor::task]
pub async fn storage_task(mut settings: Settings) {
    loop {
        match STORE_CHANNEL.receive().await {
            StoreRequest::Config(config) => settings.stored.config = Some(config),
            StoreRequest::Bond(bond) => settings.stored.bond = Some(bond),
        }
        let _ = settings.save();
    }
}
