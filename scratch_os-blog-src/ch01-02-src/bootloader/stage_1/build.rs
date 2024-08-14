fn main()
{
    println!("cargo:rustc-link-arg-bins=--script=linker.ld");
}