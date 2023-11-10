use std::path::Path;

use glob::glob;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    // Either take the path from environment or use the provided path
    let suite_path = match option_env!("REFERENCING_SUITE") {
        Some(v) => v.to_string(),
        None => parse_macro_input!(args as LitStr).value(),
    };
    // Find all test case files within the suite
    let paths = glob(&format!("{suite_path}/*/**/*.json"))
        .expect("Invalid pattern")
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to glob");
    // Generate test functions that accept a path to test case file as its input
    let input_fn = parse_macro_input!(input as ItemFn);
    let input_fn_name = &input_fn.sig.ident;

    let test_fns = paths.iter().map(|path| {
        let spec = normalize_path(path.parent().expect("Missing parent"));
        let case = normalize_path(path);
        let test_fn_name = syn::Ident::new(
            &format!("{input_fn_name}_{spec}_{case}"),
            input_fn.sig.ident.span(),
        );
        let path = path.display().to_string();
        quote! {
            #[test]
            fn #test_fn_name() {
                #input_fn_name (std::path::PathBuf::from(#path));
            }
        }
    });

    quote! {
        #input_fn

        #(#test_fns)*
    }
    .into()
}

/// Adjust the file stem so it is usable as an idntifier.
fn normalize_path(path: &Path) -> String {
    path.file_stem()
        .expect("Missing file stem")
        .to_string_lossy()
        .replace('-', "_")
        .to_ascii_lowercase()
}
