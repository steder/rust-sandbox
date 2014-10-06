fn main() {
    let someone_elses_list = ["hello", "world"];

    // What's the making immutable list declaration like this 'mut'?
    //let mut my_list = ["hello"];
    //my_list.push("mike");
    // the `my_list` variable is mutable, but I can't mutate the list it points at.  So what we're saying
    // here is that my_list is mutable and can be pointed at another
    // value, but we're not saying anything about the list being mutable.
    //
    // I kind of feel like the default list implementation should be mutable and I wonder if that's possible.
    //
    // If I just want this to work I should change the above to a vec and then push onto it:
    //
    // That said, I can't mutate my_list, the mutable vector, unless I make the variable
    // mutable which emboldens me to think that really you want to have a a macro for list
    // initialization that just does the "right" thing based on mutability of the declaration:
    let mut my_list = vec!["Hello"];
    my_list.push("Mike");

    // I'm thinking based on vec! (http://doc.rust-lang.org/std/macro.vec!.html)
    // I could create a list! macro that either creates a vec or a list
    // depending on the mutability of the variable.
    //
    // The goal would be to make:
    //
    //     let mut my_list = vec!["Hello"];
    //
    // be the same as:
    //     let mut my_list = ["Hello"];
    //
    // Of course, this may be silly, maybe explicit is better than implicit. ;-)
    // I just think a mutable reference to an immutable variable makes little sense
    // to me at the moment.  Perhaps if I'd written more C/C++ code in the last decade
    // I'd feel differently.

    for x in someone_elses_list.iter() {
        println!("someone elses item: {}", x);
    }

    for x in my_list.iter() {
        println!("my items: {}", x);
    }
}
