use my_parser_test_crates_io_1::*;
pub fn main() {
    let parsed_data = list_parser::list("[1,1,2,3,5,8]");
    dbg!(&parsed_data);
    assert_eq!(parsed_data, Ok(vec![1, 1, 2, 3, 5, 8]));
}
