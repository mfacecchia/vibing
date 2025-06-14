pub fn verbose_print(verbose: bool, message: &str) -> () {
    if verbose {
        println!("{message}");
    }
}
