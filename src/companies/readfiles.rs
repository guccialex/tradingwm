
use super::*;


/// Reads a file and returns its contents as a vector of rows,
/// where each row is a vector of trimmed, tab-separated fields.
pub fn read_tab_separated_values_from_file(file_path: &str) -> Result<Vec<Vec<String>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let row = line.split('\t')
                      .map(|s| s.trim().to_string())
                      .collect();
        rows.push(row);
    }
    Ok(rows)
}

/// Reads the starting company symbols from the specified file.
pub fn read_starting_company_info() -> Vec<CompanySymbol> {
    let rows = read_tab_separated_values_from_file("data/snpcurrent.txt")
        .unwrap_or_default();
    let mut companies = Vec::new();
    for row in rows {
        if row.len() < 4 {
            continue;
        }
        companies.push(CompanySymbol(row[0].clone()));
    }
    companies
}

/// Reads SNP replacements from the specified file.
/// Each row is expected to contain at least four fields:
/// date, symbol to remove, (unused), symbol to add.
pub fn read_snp_replacements() -> Vec<(Date, CompanySymbol, CompanySymbol)> {
    let rows = read_tab_separated_values_from_file("data/snpchanges.txt")
        .unwrap_or_default();
    let mut replacements = Vec::new();
    for row in rows {
        if row.len() < 4 {
            continue;
        }
        let date = Date::from_string(&row[0]);
        let removed = CompanySymbol(row[1].clone());
        let added = CompanySymbol(row[3].clone());
        replacements.push((date, removed, added));
    }
    replacements
}
