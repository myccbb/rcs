use proc_macro::{self, TokenStream};

#[proc_macro_derive(Row_old)]
pub fn impl_row(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    let fields_named = get_fields_named(&input).unwrap();
    for (_, field) in fields_named.iter() {
        println!("field: {:?}", field);
    }
    let c = fields_named.values().map(|field| {
        let field_name = &field.field_name;
        let field_name_ident = quote::format_ident!("{}", field_name);
        let value_enum = match field.type_name.as_str() {
            "i64" => "Integer",
            "f64" => "Real",
            "String" => "Text",
            _ => "Unknown",
        };
        let value_enum_ident = quote::format_ident!("{}", value_enum);
        // quote::quote! {
        //     let name_str = #field_name;
        //     println!("fields: {}: {:?}", name_str, self.#field_name_ident);
        //     if let sqlite::Value::#value_enum_ident(v) = value {
        //         self.#field_name_ident = v;
        //     }
        // }
        quote::quote!(
            #field_name => {
                if let sqlite::Value::#value_enum_ident(v) = value {
                    self.#field_name_ident = v;
                }
            }
        )
    });
    let struct_name = get_struct_name(&input);
    let struct_name_ident = syn::Ident::new(&struct_name, input.ident.span());
    // println!("input struct: {:?}", input);
    quote::quote! {
        impl sqlite::Row for #struct_name_ident {
            fn update_field(&mut self, column_name: &str, value: sqlite::Value) -> &mut Self {
                match column_name {
                #(#c)*
                _ => {}
                }
                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(Row)]
pub fn impl_row_v2(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    let fields_named = get_fields_named(&input).unwrap();
    for (_, field) in fields_named.iter() {
        println!("field: {:?}", field);
    }
    let c = fields_named.values().map(|field| {
        let field_name = &field.field_name;
        let field_name_ident = quote::format_ident!("{}", field_name);
        quote::quote!(
            #field_name => {
                self.#field_name_ident = value.into();
            }
        )
    });
    let struct_name = get_struct_name(&input);
    let struct_name_ident = syn::Ident::new(&struct_name, input.ident.span());
    // println!("input struct: {:?}", input);
    quote::quote! {
        impl sqlite::Row for #struct_name_ident {
            fn update_field(&mut self, column_name: &str, value: sqlite::Value) -> &mut Self {
                use sqlite::Value;
                match column_name {
                #(#c)*
                _ => {}
                }
                self
            }
        }
    }
    .into()
}

#[derive(Debug)]
struct Field {
    field_name: String,
    type_name: String,
}

use std::collections::HashMap;

fn get_fields_named(input: &syn::DeriveInput) -> Result<HashMap<String, Field>, Error> {
    match &input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(ref fields) => {
                let mut results: HashMap<String, Field> = HashMap::new();
                for (i, v) in fields.named.iter().enumerate() {
                    if v.ident.is_none() {
                        return Err(Error::StructFieldIsNotAvailable(i));
                    }
                    let field_name = v.ident.as_ref().unwrap().to_string();
                    let type_name;
                    println!("{:?}", v.ty);
                    match v.ty {
                        syn::Type::Path(ref type_path) => {
                            match type_path.path.segments.first() {
                                Some(v) => {
                                    type_name = v.ident.to_string();
                                }
                                None => {
                                    return Err(Error::PathSegmentsIsNone);
                                }
                            };
                        }
                        _ => {
                            return Err(Error::FieldTypeNotSupported("".into()));
                        }
                    };
                    results.insert(
                        field_name.clone(),
                        Field {
                            field_name,
                            type_name,
                        },
                    );
                }
                Ok(results)
            }
            syn::Fields::Unnamed(_) => Err(Error::StructFieldsIsNotNamed("unnamed".into())),
            syn::Fields::Unit => Err(Error::StructFieldsIsNotNamed("unit".into())),
        },
        syn::Data::Enum(_) => Err(Error::InputObjectIsNotStruct("enum".into())),
        syn::Data::Union(_) => Err(Error::InputObjectIsNotStruct("union".into())),
    }
}

fn get_struct_name(input: &syn::DeriveInput) -> String {
    input.ident.to_string()
}

#[derive(Debug)]
enum Error {
    InputObjectIsNotStruct(String),
    StructFieldsIsNotNamed(String),
    StructFieldIsNotAvailable(usize),
    FieldTypeNotSupported(String),
    PathSegmentsIsNone,
}
