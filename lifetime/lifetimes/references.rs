fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+


//owner has to be alive for as long as the reference is alive

//here we leave the scope so x doesnt live long enough (past the b scope)
//legal and common in c :(