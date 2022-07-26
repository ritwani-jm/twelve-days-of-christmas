fn main() {
    let gifts = ["A partridge in a pear tree", "Two turtledoves", "Three French hens", "Four calling birds", "Five gold rings (five golden rings)", "Six geese a-laying", 
                    "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"]; 

    let mut index = 0;
    while index < 12 {
        println!("On the {} day of Christmas, my true love sent to me", days[index]); 

        if index > 0 {
            let mut i = index;
            while i > 0 {
                println!("{}", gifts[i]); 
                i -= 1; 
            }
            println!("And {}", gifts[i]); 
        }
        else {
            println!("{}", gifts[index]);
            
        }
        index += 1;
    }
} 