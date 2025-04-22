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

  /*
    The function bodies above have the same code, we can eliminate the duplication by introducing a generic type parameter
    in a single function.
    To parameterize the types in a new single function , need to name the type parameter, just as we do for the value parameters to a function.
    You can use any identifier as a type parameter name. But we'll use 'T' because, by convention, type parameter names in rust are shot, often one letter, 
    and Rust's type-naming convention is UpperCamelCase. Short for type, 'T' is the default choice of most Rust programmers.

    When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means.
    Similarly, when we use a type parameter name in a function signature, we have to declare a type parameter name before we use it. To define generic 'largest' function 
   */