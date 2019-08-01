fn main() {

    let mut count = 0;

    let gifts = [
        ("First", "A partridge in a pear tree"),
        ("Second", "Two turtle doves"),
        ("Third", "Three French hens"),
        ("Forth", "Four calling birds"),
        ("Fifth", "Five gold rings, badam-pam-pam"),
        ("Sixth", "Six geese a laying"),
        ("Seventh", "Seven swans a swimming"),
        ("Eighth", "Eight maids a milking"),
        ("Ninth", "Nine ladies dancing"),
        ("Tenth", "Ten lords a leaping"),
        ("Eleventh", "Eleven pipers piping"),
        ("Twelfth", "12 drummers drumming")
    ];

    while count < 12 {

        println!("For the {} day of christmas my true love gave to me...", gifts[count].0);

        if count == 0 {
            println!("{}", gifts[count].1)
        }
        else {
            for i in (0..=count).rev() {
                if i == 0 {
                    println!("And {}", gifts[i].1)
                }
                else {
                    println!("{}", gifts[i].1)
                }
            }
        };

        println!("\n\n");
        count += 1;

    }
}
