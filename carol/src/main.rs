
fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "A partridge in a pear tree \n",
        "Two turtle doves, and \n",
        "Three french hens \n",
        "Four calling birds \n",
        "Five golden rings \n",
        "Six geese a-laying \n",
        "Seven swans a-swimming \n",
        "Eight maids a-milking \n",
        "Nine ladies dancing \n",
        "Ten lords a-leaping \n",
        "Eleven pipers piping \n",
        "Twelve drummers drumming \n",
    ];

    let mut index = 0;

    for day in days {
        let mut beggining: String = "On the ".to_owned();
        beggining.push_str(day);

        let mut present: String = presents[index].to_owned();

        if index > 0 {
            for i in (0..index).rev() {
                present.push_str(presents[i])
            }
        }

        index += 1;
        println!("{beggining} day of Christmas, my \ntrue love sent to me \n{present}")
    }
}
