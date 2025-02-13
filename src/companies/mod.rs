use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use crate::Date;
use crate::convert_date_to_epoch;

mod readfiles;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanySymbol(String);


pub struct SNPData {
    current_companies: Vec<CompanySymbol>,
    // Each tuple contains: (date, company removed at this date, company added at this date)
    replacements: Vec<(Date, CompanySymbol, CompanySymbol)>,
}

impl SNPData {

    pub fn new() -> SNPData{
        let starting_companies = readfiles::read_starting_company_info();
        //println!("Starting companies: {:?}", starting_companies);    
        let replacements = readfiles::read_snp_replacements();
        //println!("Replacements: {:?}", replacements);
        SNPData {
            current_companies: starting_companies,
            replacements,
        }
    }


    pub fn get_companies_at_date(&self, date_str: &str) -> Vec<CompanySymbol>{
        let target_epoch = convert_date_to_epoch(date_str).unwrap();
        let mut companies: HashSet<CompanySymbol> =
            self.current_companies.iter().cloned().collect();

        let mut replacements = self.replacements.clone();
        // Sort replacements descending by date.
        replacements.sort_by(|a, b| b.0.cmp(&a.0));

        for (date, removed, added) in replacements {
            if date.0 < target_epoch {
                break;
            }
            companies.remove(&removed);
            companies.insert(added);
        }

        let initial: HashSet<CompanySymbol> =
            self.current_companies.iter().cloned().collect();
        let added_companies: HashSet<_> = companies.difference(&initial).collect();
        let removed_companies: HashSet<_> = initial.difference(&companies).collect();
        println!("Added companies: {:?}", added_companies);
        println!("");
        println!("Removed companies: {:?}", removed_companies);
        println!("");

        companies.into_iter().collect()
    }



}



