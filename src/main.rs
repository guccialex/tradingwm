
mod readcompanies;

mod date;
pub use date::convert_date_to_epoch;

fn main() {
    println!("Hello, world!");

    readcompanies::main();
}




// pub struct CompanyList{
//     companies: Vec<String>
// }

// fn get_current() -> CompanyList{



// }