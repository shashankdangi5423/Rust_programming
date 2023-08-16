//Simple :: Write a function which takes a reference of a string and print it.
fn print_ref_string(str:&str) {
    println!("String reference passes is {}",str);
}

//Simple :: Write a function which takes a reference of the string and a 
//          char and returns number of times that char is present in that string.
fn print_char_len(str :&str, c :char) -> u32 {
    let mut len: u32 = 0;

    for i in str.chars() {

        if i == c {
            len = len +1;
        }
    }

    return len;
}


//Simple :: Write a function which takes two references of string and 
//          a concanated 
fn conc_string(str1 :&str, str2 :&str) -> String {

    str1.to_string() + str2

}


//Simple :: Write a function which takes a reference of string and a unicode 
//          and returns number of times it got repeated
fn len_from_unicode(str: &str, unicode: u8) -> u32 {

    let mut len = 0;

    for i in str.bytes() {
        if i == unicode {
            len = len + 1;
        }
    }

    return  len;
}

fn main() {

    //let some_ref = "shashank";
    //print_ref_string(&some_ref);

    /*
    let some_ref = "shashank";
    let chr :char = 'k';

    let mut count :u32 = 0;

    count = print_char_len(some_ref, chr);

    println!("{}",count);
    */

    
    //let str = conc_string("shashank", "dangi");
    //println!("{}",str);


    let len = len_from_unicode("shashank",0x61);
    println!("{}",len);


}
