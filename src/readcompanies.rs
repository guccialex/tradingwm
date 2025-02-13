use std::fs::File;
use std::io::{BufRead, BufReader, Result};







pub struct Date(i64);
impl Date{

    fn from_string(string: &str) -> Date{
        
    }

}

#[derive(Debug, Clone)]
pub struct CompanySymbol(String);
pub struct SNPData{
    currentcompanies: Vec<CompanySymbol>,
    replacements: Vec<(Date, CompanySymbol, CompanySymbol)>,
}


pub fn main(){

    let starting_companies = read_starting_company_info();

    println!("Starting companies: {:?}", starting_companies);


}


pub fn read_starting_company_info() -> Vec<CompanySymbol> {
    let rows = read_tab_separated_values_from_file("data/snpcurrent.txt").unwrap_or(Vec::new());
    let mut companies = Vec::new();
    for row in rows {
        if row.len() < 4 {
            continue;
        }
        let companysymbol = CompanySymbol(row[0].clone());
        companies.push(companysymbol);
    }
    companies
}


pub fn read_snp_replacements() -> Vec<(Date, CompanySymbol, CompanySymbol)> {

    //December 23, 2024	WDAY	Workday, Inc.	AMTM	Amentum	Market capitalization change.[4]

    let rows = read_tab_separated_values_from_file("data/snpcurrent.txt").unwrap_or(Vec::new());
    let mut toreturn = Vec::new();
    for row in rows {
        if row.len() < 4 {
            continue;
        }

        let date = Date(row[0].parse().unwrap());
        let added = CompanySymbol(row[1].clone());
        let removed = CompanySymbol(row[2].clone());

        toreturn.push((date, added, removed));

    }
    toreturn
}



pub fn read_tab_separated_values_from_file(file_path: &str) -> Result<Vec<Vec<String>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let columns: Vec<String> = line
            .split('\t')
            .map(|field| field.trim().to_string())
            .collect();
        rows.push(columns);
    }
    Ok(rows)
}
