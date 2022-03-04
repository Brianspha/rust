use crate::Comparison::Equal;
use crate::Comparison::Sublist;
use crate::Comparison::Superlist;
use crate::Comparison::Unequal;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq + std::clone::Clone + std::fmt::Debug>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    if  is_equal(_first_list.to_vec(), _second_list.to_vec()) {
        return Equal;
    } else if _first_list.is_empty() ||( _first_list.len() < _second_list.len() && contains_list(_first_list, _second_list)) {
        return Sublist;
    } else if _second_list.is_empty() || contains_list(_second_list,_first_list) {
        return Superlist;
    } else {
        return Unequal;
    }
}

//@dev check if the smaller_list is contained in the bigger_list see:https://tndl.medium.com/rusts-slice-windows-is-really-cool-70d50cdc74c5
pub fn contains_list<T: PartialEq + std::clone::Clone + std::fmt::Debug>(
    bigger_list: &[T],
    smaller_list: &[T],
) -> bool {
    return smaller_list
        .windows(bigger_list.len())
        .any(|created_list| created_list == bigger_list)//@dev the windows method allows us to create slices of array and we an then use the any method to compare these sliced arrays with the smaller list
}

//@dev function accepts two generic vectors and checks if the elements contained within each vector are equal if they are we return a boolean value with true else false
pub fn is_equal<T: PartialEq + std::clone::Clone + std::fmt::Debug>(
    list_a: Vec<T>,
    list_b: Vec<T>,
) -> bool {
    let mut equal_lists = list_a.is_empty() && list_b.is_empty();//@dev defualt is to check if both vectors are empty if yes they are equal
    if list_a.len() == list_b.len() {
        let mut start_index = 0;
        while start_index < list_a.len() {
            if list_a[start_index as usize] == list_b[start_index as usize] {
                equal_lists = true;
            } else {
                equal_lists = false;
                break;
            }
            start_index += 1;
        }
    }
    return equal_lists;
}
