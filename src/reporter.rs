use crate::analyzer::FootprintSummary;
use crate::types::StorageKeyDefinition;

pub struct LayoutReporter;

impl LayoutReporter {
    pub fn format_ascii(keys: &[StorageKeyDefinition], summary: &FootprintSummary) -> String {
        let mut out = String::new();
        out.push_str("┌─── Soroban Storage Key Memory Layout ───\n");
        out.push_str(&format!(
            "│ Total Keys: {} | Total Footprint: {} bytes (Avg: {:.1} B/key)\n",
            summary.total_keys, summary.total_bytes, summary.average_key_size_bytes
        ));
        out.push_str(&format!(
            "│ Instance: {} B | Persistent: {} B | Temporary: {} B\n",
            summary.instance_bytes, summary.persistent_bytes, summary.temporary_bytes
        ));
        out.push_str("├─── Key Entries ───\n");
        for (i, k) in keys.iter().enumerate() {
            out.push_str(&format!(
                "│ [{:02}] [{}] {} ({} bytes)\n",
                i + 1,
                k.domain,
                k.source_contract,
                k.key.footprint_size()
            ));
        }
        out.push_str("└─── End of Report ───\n");
        out
    }

    pub fn format_csv_footprint(keys: &[StorageKeyDefinition]) -> String {
        let mut out = String::from("index,contract,domain,key_size_bytes\n");
        for (i, k) in keys.iter().enumerate() {
            out.push_str(&format!(
                "{},{},{},{}\n",
                i + 1,
                k.source_contract,
                k.domain,
                k.key.footprint_size()
            ));
        }
        out
    }
}
