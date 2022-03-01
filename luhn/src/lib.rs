//@dev cconverst a character into a i32 number
pub fn convert_to_i32(character_number: &char)-> i32 {
  let num = character_number.to_string().parse::<i32>();
    match num {
        Ok(val) => character_number.to_string().parse::<i32>().unwrap(),
        Err(why) => -1,
    }
    
}
//@dev checks if the given character is a valid number
pub fn is_valid_number(character_number: &char)-> bool {
  let num = character_number.to_string().parse::<i32>();
    match num {
        Ok(val) => true,
        Err(why) => false,
    }
    
}
//@dev converts all elements inside vector into numbers
pub fn convert_characters_to_numbers(character_array: &Vec<char>) -> Vec<i32> {
    let mut start_index:usize=0;
    let mut temp_character_array:Vec<i32>=vec![];
    while start_index < character_array.len() {
        temp_character_array.push(convert_to_i32(&character_array[start_index]));
        start_index=start_index+1;
    }
    println!("converted numbers: {:?} ",temp_character_array);
    return temp_character_array;
}
pub fn add_all_array_item (number_array: &Vec<i32>) -> i32 {
    let sum =number_array.iter().sum();
    println!("summed array: {:?}",sum);
    return sum;
}
//@dev checks if the given character array only contains valid numbers
pub fn check_if_contains_numbers_only(character_array: &Vec<char>) -> bool {
    let mut start_index:usize=0;
    let mut is_valid:bool=true;
    while start_index < character_array.len() {
        if !&is_valid_number(&character_array[start_index])
        {
           is_valid=false;
           break;
        }
        start_index=start_index+1;
    }
    return is_valid;
}
//@dev doubles every second element in the given array
pub fn double_every_second_element(number_array:&Vec<i32>) -> Vec<i32>{
     let mut start_index:usize=0;
     let mut temp_number_array:Vec<i32>=vec![];
    while start_index < number_array.len() {
        if start_index % 2 !=0 {
            let mut doubled_number = &number_array[start_index] * 2;
            if doubled_number>9 {
                doubled_number=doubled_number-9;
            }
            println!("doubled number: {:?} index {:?} current_number {:?} ",doubled_number,start_index,number_array[start_index]);
            temp_number_array.push(doubled_number);
        }
        else{
            temp_number_array.push(number_array[start_index]);
            println!("not doubled: {:?} ",number_array[start_index]);
        }
        start_index=start_index+1;
    }
    println!("results of doubling: {:?} ",temp_number_array);
    return temp_number_array;
}

/**
*@dev function to determine if a given credit card number is valid
 @dev step1 check if the given string is empty if yes return false
 @dev step2 split the given string by empty character into a vector of type string
 @dev step3 convert the string vector into a string
 @dev step4 split the new string into an array of characters
 @dev step5 create a temp copy of the new character array
 @dev step6 check if the new array only contains numbers only if not assignn is_valid to false
 @dev step7 if array contains numbers only we then reverse it
 @dev step8 convert all characters within the character vector into numbers
 @dev step9 double every second item in the converted array
 @dev step10 sum all numbers in vector
 @dev step11 check if the sum is divisible by 10 perfectly
 @returns true or false
*/
pub fn is_valid(code: &str) -> bool {
    let mut is_valid:bool=false;
    if code.chars().count() ==0 ||code.chars().count() ==1 {
        return is_valid;
    }
    let input: Vec<&str>  =code.split(' ').collect();//@dev split by empty chracter
    let string_array=String::from_iter(input);//@dev convert to string
    let character_vector: Vec<char> = string_array.chars().collect();//@dev split into array
    let mut character_vector_temp: Vec<char> =character_vector;
    let mut number_vector_temp: Vec<i32> =vec![];
    is_valid=check_if_contains_numbers_only(&character_vector_temp) && character_vector_temp.len()!=1;
    if is_valid{
        character_vector_temp=character_vector_temp.into_iter().rev().collect();
        number_vector_temp=convert_characters_to_numbers(&character_vector_temp);
        number_vector_temp=double_every_second_element(&number_vector_temp);
        let summed_array=add_all_array_item(&number_vector_temp);
        is_valid=summed_array%10==0;
    }
    return is_valid;
}