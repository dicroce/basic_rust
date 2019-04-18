
pub fn run() {

    {
        // Primitives have copy semantics. If you assign a variable to another, they both contain the same value.
        let arr1 = [1,2,3];
        let arr2 = arr1;

        println!("Values: {:?}", (arr1, arr2));
    }


    {
        // Non primitives (in this case vector) have default move semantics

        // Vectors
        let vec1 = vec![1,2,3];
        let vec2 = vec1; // at this point vec1 is empty (and would be a compile error if you used it).

        println!("Values: {:?}", (vec2));
    }

    {
        // References
        let vec1 = vec![1,2,3];
        let vec2 = &vec1; // by adding a reference & we are changing the type returned by the right hand side
                          // and thuse we are changing the type of vec2. This does not move the value out of vec1
                          // instead it simply creates vec2 as a reference to vec1.

        println!("Values: {:?}", (&vec1, vec2)); // NOTE: passing a non primite to a function invokes move semantics, so in
                                                 // this case we cannot simply pass vec1 to println! because vec1 is already
                                                 // borrowed by vec2. What we can do is borrow vec1 and pass that.
                                                 // The trickiest think about this is that this would have been allowed had we
                                                 // not borrowed vec1 to create vec2.
                                                 // BUT, that's not the whole story! The real entity forcing the & in the println
                                                 // is the tuple construction.
    }

    {
        let x = String::from("hello");
        let y = &x;
        println!("{}", x); // It doesn't require & because it is a macro. Try using {:?} and create a tuple with both values (x,y).
                           // it will fail as above.
        println!("{}", y);
    }
}
