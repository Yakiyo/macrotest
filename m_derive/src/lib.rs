use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mkdoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let ast = syn::parse(item).unwrap();
    let vec: Vec<_> = attr
        .into_iter()
        .filter(|x| x.to_string().as_str() != ",")
        .map(|x| x.to_string())
        .collect();
    assert!(vec.len() >= 4, "received less then 4 attributes");
    let module = &vec[0];
    let name = &vec[1];
    let output = vec.last().unwrap();
    let inputs = &vec[2..(vec.len() - 1)];
    println!("mod: {module}\nname: {name}\ninputs:{inputs:#?}\noutputs: {output}");
    let path = format!("./doc/{module}/{name}.md"); 
    std::fs::create_dir_all(format!("./doc/{module}")).unwrap();
    std::fs::write(
        path,
        format!("name: {name}\ninputs:{inputs:#?}\noutputs: {output}",),
    )
    .unwrap();
    item
}
