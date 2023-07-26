use {
    borsh_custom_derive::{BorshSchema0_9, BorshSchema0_10},
    borsh_0_9,
    borsh_0_10,
};

#[derive(Debug, BorshSchema0_9, BorshSchema0_10)]
struct S {
    #[allow(dead_code)]
    v1: u8,
    #[allow(dead_code)]
    v2: u32,
}

macro_rules! borsh_schema_common_functions {
    () => {
        pub fn get_packed_len<S: BorshSchema>() -> usize {
            let BorshSchemaContainer { declaration, definitions } =
                &S::schema_container();
            get_declaration_packed_len(declaration, definitions)
        }

        pub fn get_declaration_packed_len(
            declaration: &str,
            definitions: &HashMap<Declaration, Definition>,
        ) -> usize {
            match definitions.get(declaration) {
                Some(Definition::Array { length, elements }) => {
                    *length as usize * get_declaration_packed_len(elements, definitions)
                }
                Some(Definition::Enum { variants }) => {
                    1 + variants
                        .iter()
                        .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
                        .max()
                        .unwrap_or(0)
                }
                Some(Definition::Struct { fields }) => match fields {
                    Fields::NamedFields(named_fields) => named_fields
                        .iter()
                        .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
                        .sum(),
                    Fields::UnnamedFields(declarations) => declarations
                        .iter()
                        .map(|declaration| get_declaration_packed_len(declaration, definitions))
                        .sum(),
                    Fields::Empty => 0,
                },
                Some(Definition::Sequence {
                    elements: _elements,
                }) => panic!("Missing support for Definition::Sequence"),
                Some(Definition::Tuple { elements }) => elements
                    .iter()
                    .map(|element| get_declaration_packed_len(element, definitions))
                    .sum(),
                None => match declaration {
                    "bool" | "u8" | "i8" => 1,
                    "u16" | "i16" => 2,
                    "u32" | "i32" => 4,
                    "u64" | "i64" => 8,
                    "u128" | "i128" => 16,
                    "nil" => 0,
                    _ => panic!("Missing primitive type: {declaration}"),
                },
            }
        }
    }
}

mod for_borsh_0_9 {
    use {
        borsh_0_9::{
            schema::{BorshSchema, BorshSchemaContainer, Declaration, Definition, Fields},
        },
        std::collections::HashMap,
    };

    borsh_schema_common_functions!();
}

mod for_borsh_0_10 {
    use {
        borsh_0_10::{
            schema::{BorshSchema, BorshSchemaContainer, Declaration, Definition, Fields},
        },
        std::collections::HashMap,
    };

    borsh_schema_common_functions!();
}

fn main() {
    let s = S {
        v1: 7,
        v2: 382,
    };

    let v_0_9_size = for_borsh_0_9::get_packed_len::<S>();
    let v_0_10_size = for_borsh_0_10::get_packed_len::<S>();

    println!("{s:?}");
    println!("v 0.9 size:  {v_0_9_size}");
    println!("v 0.10 size: {v_0_10_size}");
}
