use super::fail::UnwrapOrFail;

pub(crate) fn get_partition(partitions_raw: &[u8], index: usize) -> PartitionTableEntry {
    const ENTRY_SIZE: usize = 16;

    let offset = index * ENTRY_SIZE;
    let buffer = partitions_raw.get(offset..).unwrap_or_fail(b'c');

    let bootable_raw = *buffer.first().unwrap_or_fail(b'd');
    let bootable = bootable_raw == 0x80;

    let partition_type = *buffer.get(4).unwrap_or_fail(b'e');

    let lba = u32::from_le_bytes(
        buffer
            .get(8..)
            .and_then(|slice| slice.get(..4))
            .and_then(|slice| slice.try_into().ok())
            .unwrap_or_fail(b'f'),
    );

    let len = u32::from_le_bytes(
        buffer
            .get(12..)
            .and_then(|slice| slice.get(..4))
            .and_then(|slice| slice.try_into().ok())
            .unwrap_or_fail(b'g'),
    );

    PartitionTableEntry {
        bootable,
        partition_type,
        logical_block_address: lba,
        number_of_sectors: len,
    }
}

pub(crate) struct PartitionTableEntry {
    pub(crate) bootable: bool,
    pub(crate) partition_type: u8,
    pub(crate) logical_block_address: u32,
    pub(crate) number_of_sectors: u32,
}
