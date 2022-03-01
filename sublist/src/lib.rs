use crate::Comparison::Equal;
use crate::Comparison::Sublist;
use crate::Comparison::Superlist;
use crate::Comparison::Unequal;
use std::collections::HashSet;

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
    let mut source_a: &[T] = &[];
    let mut source_b: &[T] = &[];

    if _first_list.to_vec().len() == 0 || _second_list.to_vec().len() == 0 {
        return Equal;
    } else if _first_list.to_vec().len() > _second_list.to_vec().len() {
        source_a = _first_list;
        source_b = _second_list;
    } else {
        source_b = _first_list;
        source_a = _second_list;
    }
    let start_index = get_start_sub_list_index(&source_a.to_vec(), &source_b.to_vec()); //@dev get the index where the sublist or superlist starts
    if start_index == -1 {
        //@dev if we dont find any start index of the items in the smaller list return unequal
        return Unequal;
    }
    println!("start_index: {:?}", start_index);
    let difference: usize = ((source_a.to_vec().len()
        - (source_b.to_vec().len() + start_index as usize)) as usize)
        .try_into()
        .unwrap(); //@dev get the difference between the start and end of where the sublist or superlist starts
    let exact_match =
        &source_a[start_index as usize..(source_b.to_vec().len() + start_index as usize) as usize]; //@dev slice the array only leaving the sublist or superlist
    let arrays_equal = is_equal(source_a.to_vec(), source_b.to_vec());
    if start_index as usize != difference as usize {
        //@dev here we only interested if we havent sacrificed unqual number of items from both ends if we have then we defs know this is a sublist
        return Sublist;
    } else if &source_a.to_vec().len() == &source_b.to_vec().len() && arrays_equal {
        return Equal;
    }   else  if &source_a.to_vec().len() != &source_b.to_vec().len() && &source_a.to_vec().len() > &source_b.to_vec().len() {//@dev if we have sacrificed an equal number of array items from both ends then we defs know the bigger list is a superlist
        println!("superlist");
       return Superlist;
    }
    else{
        return Unequal;
    }
}
//@dev function accepts two generic vectors and returns the index where the smaller list within the larger list is if there is any
pub fn get_start_sub_list_index<T: PartialEq + std::clone::Clone + std::fmt::Debug>(
    bigger_list: &Vec<T>,
    smaller_list: &Vec<T>,
) -> i64 {
    for i in 0..bigger_list.len() - smaller_list.len() + 1 {
        // last indices of bigger_list are too far right to get a match
        let mut j = 0;
        while j < smaller_list.len() {
            // check every char of needle
            if smaller_list[j] != bigger_list[i + j] {
                // doesn't match
                break; // try the next i
            }
            j += 1; // else: match so far
        }
        if j == smaller_list.len() {
            // no break: a full match was found
            return i as i64;
        }
    }
    return -1; // not a single full match
}
//@dev function accepts two generic vectors and checks if the elements contained within each vector are equal if they are we return a boolean value with true else false
pub fn is_equal<T: PartialEq + std::clone::Clone + std::fmt::Debug>(
    list_a: Vec<T>,
    list_b: Vec<T>,
) -> bool {
    let mut equal_lists = false;
    if list_a.len() == list_b.len() {
        let mut start_index = 0;
        while start_index < list_a.len() {
            if list_a[start_index as usize] == list_b[start_index as usize] {
                equal_lists = true;
            } else {
                equal_lists = false;
                break;
            }
            start_index = start_index + 1;
        }
    }
    return equal_lists;
}
