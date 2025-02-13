
mod companies;

mod date;
pub use date::Date;
pub use date::convert_date_to_epoch;

fn main() {
    println!("Hello, world!");


    let snp_data = companies::SNPData::new();
    let snp_500_companies_at_date = snp_data.get_companies_at_date("September 24, 2024");

    println!("Companies in the S&P 500 on September 23, 2024: {:?} {:?}", snp_500_companies_at_date, snp_500_companies_at_date.len());

}
