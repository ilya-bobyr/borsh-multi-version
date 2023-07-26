use {
    borsh_schema_derive_internal_0_10, borsh_schema_derive_internal_0_9,
    proc_macro::TokenStream,
    proc_macro2::{Ident, Span},
    syn::{parse, Error, ItemEnum, ItemStruct, ItemUnion},
};

#[proc_macro_derive(BorshSchema0_9)]
pub fn borsh_schema_0_9(input: TokenStream) -> TokenStream {
    use borsh_schema_derive_internal_0_9::{process_enum, process_struct};

    let crate_name = Ident::new("borsh_0_9", Span::call_site());

    let res = if let Ok(input) = parse::<ItemStruct>(input.clone()) {
        process_struct(&input, crate_name)
    } else if let Ok(input) = parse::<ItemEnum>(input.clone()) {
        process_enum(&input, crate_name)
    } else if parse::<ItemUnion>(input).is_ok() {
        Err(Error::new(
            Span::call_site(),
            "Borsh schema does not support unions yet.",
        ))
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}

#[proc_macro_derive(BorshSchema0_10)]
pub fn borsh_schema_0_10(input: TokenStream) -> TokenStream {
    use borsh_schema_derive_internal_0_10::{process_enum, process_struct};

    let crate_name = Ident::new("borsh_0_10", Span::call_site());

    let res = if let Ok(input) = parse::<ItemStruct>(input.clone()) {
        process_struct(&input, crate_name)
    } else if let Ok(input) = parse::<ItemEnum>(input.clone()) {
        process_enum(&input, crate_name)
    } else if parse::<ItemUnion>(input).is_ok() {
        Err(Error::new(
            Span::call_site(),
            "Borsh schema does not support unions yet.",
        ))
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}
