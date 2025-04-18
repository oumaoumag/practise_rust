/* 
    We use generics to create definiations for items like functions signatures or structs, which we can
    then use with many different concrete data types. 
 */

 // In functions Definations
 /*
    When defining a function that uses generica, we place the generics in the signature of the function
    where we would usually specify the data types of the parameters and return value.
     Doing so makes our code more flexible and provides more functionality to callers of our functions while
     preventing code duplication.
  */

  fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
  }

  fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in  list {
        if item > largest {
            largest =  item;
        }
    }
    largest
  }

  fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest character is {result}");
  }