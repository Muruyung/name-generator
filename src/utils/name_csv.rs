use std::{env::current_dir, fs::File};
use rand::Rng;

#[derive(Clone,Debug,Default,serde::Deserialize)]
pub struct NameCSV {
    sex: String,
    name: String,
    alt_spellings: String,
    n_sum: u64,
    year_min: u32,
    year_max: u32,
    year_pop: u32,
    biblical: Option<i8>,
    palindrome: Option<i8>,
    phones: String,
    first_letter: String,
    stresses: String,
    syllables: Option<u32>,
    alliteration_first: Option<u32>,
    unisex: Option<i8>,
}

impl NameCSV {
    pub fn get_sex(self) -> String {
        self.sex
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn get_alt_spellings(self) -> String {
        self.alt_spellings
    }

    pub fn get_n_sum(self) -> u64 {
        self.n_sum
    }

    pub fn get_year_min(self) -> u32 {
        self.year_min
    }

    pub fn get_year_max(self) -> u32 {
        self.year_max
    }

    pub fn get_year_pop(self) -> u32 {
        self.year_pop
    }

    pub fn get_biblical(self) -> Option<i8> {
        self.biblical
    }

    pub fn get_palindrome(self) -> Option<i8> {
        self.palindrome
    }

    pub fn get_phones(self) -> String {
        self.phones
    }

    pub fn get_first_letter(self) -> String {
        self.first_letter
    }

    pub fn get_stresses(self) -> String {
        self.stresses
    }

    pub fn get_syllables(self) -> Option<u32> {
        self.syllables
    }

    pub fn get_alliteration_first(self) -> Option<u32> {
        self.alliteration_first
    }

    pub fn get_unisex(self) -> Option<i8> {
        self.unisex
    }

}

pub fn dataset_name(year: u32, gender: String) ->  Result<NameCSV, csv::Error> {
    let current = current_dir()?;
    let path = current.display().to_string() + "/src/utils/all-names.csv";
    let data_res = File::open(path)?;

    let mut reader = csv::Reader::from_reader(data_res);
    let mut arr_result: Vec<NameCSV> = Vec::new();
    for record in reader.deserialize() {
        arr_result.push(record?)
    }

    arr_result = arr_result.
        into_iter().
        filter(|y| y.year_min <= year).
        filter(|g| g.sex == gender).collect();

    let mut rng = rand::thread_rng();
    let generator: usize = rng.gen_range(0..arr_result.len());

    Ok(arr_result[generator].clone())
}