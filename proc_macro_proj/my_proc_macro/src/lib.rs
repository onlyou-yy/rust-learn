use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_first_attr_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("--------attr-------");
    eprintln!("{:#?}", attr);
    eprintln!("--------item-------");
    eprintln!("{:#?}", item);
    item
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    "fn hello() {println!(\"hello world\");}".parse().unwrap()
}
