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

    pub fn format_html(keys: &[StorageKeyDefinition], summary: &FootprintSummary) -> String {
        let mut out = String::from("<!DOCTYPE html>\n<html>\n<head>\n<title>Soroban Storage Layout</title>\n");
        out.push_str("<style>\nbody { font-family: monospace; background: #0f172a; color: #f8fafc; padding: 20px; }\n");
        out.push_str(".bar { height: 20px; border-radius: 4px; margin: 4px 0; display: flex; }\n");
        out.push_str(".instance { background: #3b82f6; }\n.persistent { background: #10b981; }\n.temp { background: #f59e0b; }\n");
        out.push_str("</style>\n</head>\n<body>\n");
        out.push_str(&format!("<h1>Soroban Storage Footprint (Total: {} bytes)</h1>\n", summary.total_bytes));
        out.push_str(&format!("<p>Instance: {} B | Persistent: {} B | Temporary: {} B</p>\n", summary.instance_bytes, summary.persistent_bytes, summary.temporary_bytes));
        out.push_str("<div class=\"bar\">\n");
        if summary.total_bytes > 0 {
            let inst_pct = (summary.instance_bytes as f64 / summary.total_bytes as f64) * 100.0;
            let pers_pct = (summary.persistent_bytes as f64 / summary.total_bytes as f64) * 100.0;
            let temp_pct = (summary.temporary_bytes as f64 / summary.total_bytes as f64) * 100.0;
            out.push_str(&format!("<div class=\"instance\" style=\"width: {:.1}%\"></div>\n", inst_pct));
            out.push_str(&format!("<div class=\"persistent\" style=\"width: {:.1}%\"></div>\n", pers_pct));
            out.push_str(&format!("<div class=\"temp\" style=\"width: {:.1}%\"></div>\n", temp_pct));
        }
        out.push_str("</div>\n<ul>\n");
        for k in keys {
            out.push_str(&format!("<li>[{}] {} - {} ({} bytes)</li>\n", k.domain, k.source_contract, k.key, k.key.footprint_size()));
        }
        out.push_str("</ul>\n</body>\n</html>\n");
        out
    }
}
