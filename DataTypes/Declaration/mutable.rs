fn main() {
    // constants must be computable at compile time
    const POUND_TO_OUNCE : u32 = 16;

    let mut oz = 1;
    oz = oz * POUND_TO_OUNCE;

    let lb = oz;
    let lb = oz/POUND_TO_OUNCE;

    let everything = "everything";
    let everything = everything.len();
}