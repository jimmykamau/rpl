fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    const GIFTS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {} day of Christmas, my Shpartsi gave to me", day);

        for j in (0..i + 1).rev() {
            if j == 0 && i > 0 {
                println!("And {}", GIFTS[0].to_lowercase());
                break;
            } else {
                println!("{}", GIFTS[j]);
            }
        }

        println!();
    }
}
