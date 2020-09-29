use mila::CompressionFormat;

struct SheetListEntry {
    name: String,
    field_type: String,
    metadata: Vec<u8>,
}

fn get_sheet_list_addresses(
    archive: &mila::BinArchive,
) -> std::result::Result<Vec<(usize, String)>, mila::ArchiveError> {
    let mut addresses: Vec<(usize, String)> = Vec::new();
    for (address, label) in archive.get_labels() {
        if label.starts_with("MOD_") {
            addresses.push((address, label));
        }
    }
    Ok(addresses)
}

fn parse_sheet_list(
    archive: &mila::BinArchive,
    sheet_address: usize,
) -> std::result::Result<Vec<SheetListEntry>, mila::ArchiveError> {
    let mut entries: Vec<SheetListEntry> = Vec::new();
    let count = archive.read_u32(sheet_address)?;
    for i in 0..count {
        let base = (i * 20 + 4) as usize + sheet_address;
        let name = archive
            .read_string(base)?
            .expect(&format!("Missing field name at 0x{:X}", base));
        let field_type = archive
            .read_string(base + 4)?
            .expect(&format!("Missing field type at 0x{:X}", base + 4));
        let metadata = archive.read_bytes(base + 8, 12)?.to_vec();
        entries.push(SheetListEntry {
            name,
            field_type,
            metadata,
        });
    }
    Ok(entries)
}

fn main() {
    let file = std::env::args().nth(1).expect("No input file given");
    let file_contents = std::fs::read(&file).expect("Failed to read input file");
    let real_contents = if file.ends_with(".lz") {
        let compression = mila::LZ13CompressionFormat {};
        compression
            .decompress(&file_contents)
            .expect("Failed to decompress file")
    } else {
        file_contents
    };
    let archive =
        mila::BinArchive::from_bytes(&real_contents).expect("Failed to parse bin archive");
    let addresses = get_sheet_list_addresses(&archive)
        .expect("Failed to find sheet lists");
    for (address, label) in addresses {
        let entries = parse_sheet_list(&archive, address)
            .expect("Failed to parse sheet list");
        println!("{}", label.strip_prefix("MOD_").unwrap());
        for entry in entries {
            println!("{}: {}", entry.name, entry.field_type);
        }
        println!("");
    }
}
