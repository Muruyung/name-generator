use crate::utils::name_csv;

mod utils;

fn main() {
    let year = 2050;
    let gender = "M";
    
    let first = match name_csv::dataset_name(year,gender.to_string()) {
        Ok(name) => name,
        Err(error) => panic!("Problem opening dataset name: {:?}", error),
    };

    let last = match name_csv::dataset_name(year,gender.to_string()) {
        Ok(name) => name,
        Err(error) => panic!("Problem opening dataset name: {:?}", error),
    };
    
    println!("{} {}", first.get_name(), last.get_name())
}
