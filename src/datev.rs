#![allow(dead_code)]
use crate::models::{DatevExportConfig, Invoice};
use chrono::Utc;

/// Generates a DATEV EXTF (Format Version 700 / 21) Buchungsstapel CSV
pub fn generate_datev_csv(
    invoices: &[Invoice],
    config: &DatevExportConfig,
    company_name: &str,
) -> String {
    let now = Utc::now();
    let timestamp = now.format("%Y%m%d%H%M%S000").to_string();
    let consultant_nr = config.consultant_number.unwrap_or(10001);
    let client_nr = config.client_number.unwrap_or(20002);
    let wj_beginn = format!("{}0101", config.year);
    let datum_von = format!("{}0101", config.year);
    let datum_bis = format!("{}1231", config.year);

    // SKR03 vs SKR04 revenue account mapping (19% standard VAT)
    let (konto_forderungen, gegenkonto_erloese) = match config.skr.to_lowercase().as_str() {
        "skr04" => ("1200", "4400"), // 1200 Forderungen a. LL, 4400 Erlöse 19% USt
        _ => ("1400", "8400"),       // SKR03: 1400 Forderungen a. LL, 8400 Erlöse 19% USt
    };

    let mut output = String::new();

    // 1. DATEV Header Row 1 (Metadata)
    output.push_str(&format!(
        "\"EXTF\";700;21;\"Buchungsstapel\";9;{};\"\";\"\";\"RE\";\"{}\";{};{};{};4;{};{};\"Rechnungen {}\";\"MC\";1;0;0;\"EUR\";\"\";\"\";\"\";\"\";\"\";\"\"\r\n",
        timestamp,
        company_name,
        consultant_nr,
        client_nr,
        wj_beginn,
        datum_von,
        datum_bis,
        company_name
    ));

    // 2. DATEV Header Row 2 (Columns)
    output.push_str(
        "\"Umsatz (ohne Soll/Haben-Kz)\";\"Soll/Haben-Kennzeichen\";\"WKZ\";\"Kurs\";\"Basis-Umsatz\";\"WKZ Basis-Umsatz\";\"Konto\";\"Gegenkonto (ohne BU-Schlüssel)\";\"BU-Schlüssel\";\"Belegdatum\";\"Belegfeld 1\";\"Belegfeld 2\";\"Skonto\";\"Buchungstext\";\"Postensperre\";\"Diverse Adressnummer\";\"Geschäftspartnerbank\";\"Sachverhalt\";\"Zinssperre\";\"Beleglink\";\"Festschreibung\"\r\n"
    );

    // 3. Data Rows
    for inv in invoices {
        if inv.status == "draft" {
            continue; // Skip drafts; only export sent/paid/cancelled invoices per GoBD
        }

        let is_storno = inv.invoice_number.starts_with("ST-") || inv.gross_cents < 0;
        let sh_kz = if is_storno { "H" } else { "S" }; // Haben für Gutschrift/Storno, Soll für Ausgangsrechnung
        let abs_cents = inv.gross_cents.abs();
        let eur_val = abs_cents as f64 / 100.0;
        let formatted_amount = format!("{:.2}", eur_val).replace('.', ",");

        // Format Belegdatum as DDMM
        let parts: Vec<&str> = inv.issued_date.split('-').collect();
        let belegdatum = if parts.len() == 3 {
            format!("{}{}", parts[2], parts[1]) // Day, Month
        } else {
            "0101".to_string()
        };

        let contact_name = inv.contact_name.as_deref().unwrap_or("Kunde");
        let buchungstext = if is_storno {
            format!("Storno {} - {}", inv.invoice_number, contact_name)
        } else {
            format!("Rechnung {} - {}", inv.invoice_number, contact_name)
        };

        // Truncate buchungstext to 60 chars (DATEV limit)
        let buchungstext_truncated: String = buchungstext.chars().take(60).collect();

        output.push_str(&format!(
            "\"{}\";\"{}\";\"EUR\";\"\";\"\";\"\";\"{}\";\"{}\";\"\";\"{}\";\"{}\";\"\";\"\";\"{}\";\"0\";\"\";\"\";\"\";\"0\";\"\";\"1\"\r\n",
            formatted_amount,
            sh_kz,
            konto_forderungen,
            gegenkonto_erloese,
            belegdatum,
            inv.invoice_number,
            buchungstext_truncated
        ));
    }

    output
}
