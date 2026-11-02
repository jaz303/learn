fn main() {
    let days = [
        "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (ix, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love gave to me...");
        for (iix, gift) in (&gifts[0..=ix]).iter().rev().enumerate() {
            if ix > 0 && iix == ix {
                print!("and ");
            }
            print!("{gift}");
            if ix > 0 && iix < ix {
                print!(",");
            }
            println!("");
        }
        println!("");
    }
}
