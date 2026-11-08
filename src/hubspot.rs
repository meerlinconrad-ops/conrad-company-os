#![allow(dead_code)]
use crate::models::{HubSpotImportResult, NewContact, NewDeal};

/// Simple CSV line splitter that respects quoted values
pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let delimiter = if line.contains(';') { ';' } else { ',' };

    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            d if d == delimiter && !in_quotes => {
                fields.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

/// Parses HubSpot contacts CSV content into NewContact records
pub fn parse_hubspot_contacts_csv(content: &str) -> (Vec<NewContact>, HubSpotImportResult) {
    let mut contacts = Vec::new();
    let mut errors = Vec::new();
    let mut skipped = 0;

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return (
            contacts,
            HubSpotImportResult {
                imported_count: 0,
                skipped_count: 0,
                errors: vec!["CSV-Datei ist leer".to_string()],
                summary: "Keine Zeilen in CSV gefunden.".to_string(),
            },
        );
    }

    let header_line = lines[0];
    let headers: Vec<String> = parse_csv_line(header_line)
        .into_iter()
        .map(|h| h.to_lowercase())
        .collect();

    // Find column indexes
    let idx_first_name = headers.iter().position(|h| h.contains("first name") || h == "vorname");
    let idx_last_name = headers.iter().position(|h| h.contains("last name") || h == "nachname");
    let idx_full_name = headers.iter().position(|h| h.contains("name") && !h.contains("company"));
    let idx_email = headers.iter().position(|h| h.contains("email") || h.contains("e-mail"));
    let idx_phone = headers.iter().position(|h| h.contains("phone") || h.contains("telefon") || h.contains("mobile"));
    let idx_company = headers.iter().position(|h| h.contains("company") || h.contains("unternehmen") || h.contains("firma"));
    let idx_status = headers.iter().position(|h| h.contains("status") || h.contains("lifecycle"));

    for (row_idx, line) in lines.iter().skip(1).enumerate() {
        let fields = parse_csv_line(line);
        if fields.is_empty() {
            skipped += 1;
            continue;
        }

        let name = if let (Some(f_idx), Some(l_idx)) = (idx_first_name, idx_last_name) {
            let first = fields.get(f_idx).map(|s| s.as_str()).unwrap_or("");
            let last = fields.get(l_idx).map(|s| s.as_str()).unwrap_or("");
            let full = format!("{} {}", first, last).trim().to_string();
            if full.is_empty() {
                idx_full_name.and_then(|i| fields.get(i)).cloned().unwrap_or_default()
            } else {
                full
            }
        } else if let Some(n_idx) = idx_full_name {
            fields.get(n_idx).cloned().unwrap_or_default()
        } else {
            String::new()
        };

        let email = idx_email.and_then(|i| fields.get(i)).filter(|s| !s.is_empty()).cloned();
        let phone = idx_phone.and_then(|i| fields.get(i)).filter(|s| !s.is_empty()).cloned();
        let company = idx_company.and_then(|i| fields.get(i)).filter(|s| !s.is_empty()).cloned();

        let raw_status = idx_status.and_then(|i| fields.get(i)).map(|s| s.to_lowercase()).unwrap_or_default();
        let status = if raw_status.contains("customer") || raw_status.contains("kunde") {
            "customer"
        } else if raw_status.contains("qualified") || raw_status.contains("opportunity") {
            "qualified"
        } else {
            "lead"
        };

        if name.is_empty() && email.is_none() {
            skipped += 1;
            errors.push(format!("Zeile {}: Weder Name noch E-Mail angegeben", row_idx + 2));
            continue;
        }

        let final_name = if name.is_empty() {
            email.clone().unwrap_or_else(|| "Unbekannt".to_string())
        } else {
            name
        };

        contacts.push(NewContact {
            name: final_name,
            email,
            phone,
            company,
            status: Some(status.to_string()),
            tags: Some("HubSpot-Import".to_string()),
        });
    }

    let summary = format!(
        "HubSpot-Import abgeschlossen: {} Kontakte importiert, {} übersprungen.",
        contacts.len(),
        skipped
    );

    (
        contacts.clone(),
        HubSpotImportResult {
            imported_count: contacts.len(),
            skipped_count: skipped,
            errors,
            summary,
        },
    )
}

/// Parses HubSpot Deals CSV content into NewDeal records
pub fn parse_hubspot_deals_csv(content: &str, default_contact_id: &str) -> (Vec<NewDeal>, HubSpotImportResult) {
    let mut deals = Vec::new();
    let errors = Vec::new();
    let mut skipped = 0;

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return (
            deals,
            HubSpotImportResult {
                imported_count: 0,
                skipped_count: 0,
                errors: vec!["CSV ist leer".to_string()],
                summary: "Keine Deals in CSV gefunden.".to_string(),
            },
        );
    }

    let headers: Vec<String> = parse_csv_line(lines[0])
        .into_iter()
        .map(|h| h.to_lowercase())
        .collect();

    let idx_deal_name = headers.iter().position(|h| h.contains("deal name") || h.contains("bezeichnung") || h == "name");
    let idx_amount = headers.iter().position(|h| h.contains("amount") || h.contains("wert") || h.contains("betrag"));
    let idx_stage = headers.iter().position(|h| h.contains("stage") || h.contains("phase") || h.contains("status"));
    let idx_close_date = headers.iter().position(|h| h.contains("close date") || h.contains("abschlussdatum"));

    for (row_idx, line) in lines.iter().skip(1).enumerate() {
        let fields = parse_csv_line(line);
        if fields.is_empty() {
            skipped += 1;
            continue;
        }

        let title = idx_deal_name
            .and_then(|i| fields.get(i))
            .filter(|s| !s.is_empty())
            .cloned()
            .unwrap_or_else(|| format!("Deal #{}", row_idx + 1));

        let amount_raw = idx_amount
            .and_then(|i| fields.get(i))
            .map(|s| s.replace(',', ".").replace('€', "").replace('$', "").trim().to_string())
            .unwrap_or_default();

        let value_eur: f64 = amount_raw.parse().unwrap_or(0.0);
        let value_cents = (value_eur * 100.0).round() as i64;

        let raw_stage = idx_stage
            .and_then(|i| fields.get(i))
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let stage = if raw_stage.contains("won") || raw_stage.contains("gewonnen") {
            "closed_won"
        } else if raw_stage.contains("lost") || raw_stage.contains("verloren") {
            "closed_lost"
        } else if raw_stage.contains("decision") || raw_stage.contains("contract") || raw_stage.contains("verhandlung") {
            "negotiation"
        } else if raw_stage.contains("presentation") || raw_stage.contains("proposal") || raw_stage.contains("angebot") {
            "proposal"
        } else {
            "discovery"
        };

        let close_date = idx_close_date.and_then(|i| fields.get(i)).filter(|s| !s.is_empty()).cloned();

        deals.push(NewDeal {
            contact_id: default_contact_id.to_string(),
            title,
            value_cents,
            currency: Some("EUR".to_string()),
            stage: Some(stage.to_string()),
            expected_close_date: close_date,
        });
    }

    let summary = format!(
        "HubSpot Deals importiert: {} Deals angelegt, {} übersprungen.",
        deals.len(),
        skipped
    );

    (
        deals.clone(),
        HubSpotImportResult {
            imported_count: deals.len(),
            skipped_count: skipped,
            errors,
            summary,
        },
    )
}
