extern crate proc_macro;

use darling::{
    FromDeriveInput, FromField,
    ast, util
};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(table), supports(struct_named))]
struct TableData {
    #[darling(default)]
    name: Option<String>,
    ident: syn::Ident,
    data: ast::Data<util::Ignored, TableField>,
}

#[derive(Debug, FromField)]
#[darling(attributes(table))]
struct TableField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    pk: bool,
    #[darling(default)]
    skip: bool,
}

#[proc_macro_derive(Get, attributes(table))]
#[proc_macro_error]
pub fn get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let table: TableData = TableData::from_derive_input(&derive_input).unwrap();
    let table_ident = table.ident;

    let table_name = table.name.clone().unwrap_or_else(|| table_ident.to_string().to_ascii_lowercase());

    let fields = match table.data {
        ast::Data::Struct(ref s) => s,
        _ => unreachable!()
    };

    let fields_with_pk = fields.iter().filter(|f| { f.pk }).collect::<Vec<_>>();
    
    let (pk_column_ident, pk_column_type) = match fields_with_pk.len() {
        0 => abort!(derive_input.span(),"Table '{}' has no primary key", table_name),
        1 => (fields_with_pk.first().unwrap().ident.as_ref().unwrap(), fields_with_pk.first().unwrap().ty.clone()),
        _ => abort!(derive_input.span(),"Table '{}' has more than one primary key", table_name),
    };

    let query = format!("SELECT * FROM {} WHERE {} = $1", table_name, pk_column_ident);

    let gen = quote! {
        impl #table_ident {
            pub async fn get(pool: & ::sqlx::PgPool, #pk_column_ident: #pk_column_type) -> Result<Self, ::sqlx::Error> {
                let data = ::sqlx::query_as!(
                    Self,
                    #query,
                    #pk_column_ident,
                )
                .fetch_one(pool)
                .await?;
                Ok(data)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Create, attributes(table))]
#[proc_macro_error]
pub fn create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let table: TableData = TableData::from_derive_input(&derive_input).unwrap();
    let table_ident = table.ident;

    let table_name = table.name.clone().unwrap_or_else(|| table_ident.to_string().to_ascii_lowercase());

    let fields = match table.data {
        ast::Data::Struct(ref s) => s,
        _ => unreachable!()
    };

    let fields_not_skip = fields.iter().filter(|f| { !f.skip }).collect::<Vec<_>>();

    let fn_args = fields_not_skip
        .iter()
        .map(|f| {
            let ident = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            quote!(#ident: &#ty)
        })
        .collect::<Vec<_>>();

    let field_idents = fields_not_skip.iter().map(|f| { f.ident.clone().unwrap() }).collect::<Vec<_>>();
    let field_names = fields_not_skip.iter().map(|f| { f.ident.clone().unwrap().to_string() }).collect::<Vec<_>>().join(",");
    let incremental = (1..=fields_not_skip.len()).map(|i| { format!("${}", i) }).collect::<Vec<_>>().join(",");

    let query = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING *", table_name, field_names, incremental);

    let gen = quote! {
        impl #table_ident {
            #[allow(clippy::too_many_arguments)]
            pub async fn create(pool: & ::sqlx::PgPool, #(#fn_args),*) -> Result<Self, ::sqlx::Error> {
                let data = ::sqlx::query_as!(
                    Self,
                    #query,
                    #(#field_idents),*
                )
                .fetch_one(pool)
                .await?;
                Ok(data)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Update, attributes(table))]
#[proc_macro_error]
pub fn update(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let table: TableData = TableData::from_derive_input(&derive_input).unwrap();
    let table_ident = table.ident;

    let table_name = table.name.clone().unwrap_or_else(|| table_ident.to_string().to_ascii_lowercase());

    let fields = match table.data {
        ast::Data::Struct(ref s) => s,
        _ => unreachable!()
    };

    let fields_with_pk = fields.iter().filter(|f| { f.pk }).collect::<Vec<_>>();
    
    let pk_column_ident= match fields_with_pk.len() {
        0 => abort!(derive_input.span(),"Table '{}' has no primary key", table_name),
        1 => fields_with_pk.first().unwrap().ident.as_ref().unwrap(),
        _ => abort!(derive_input.span(),"Table '{}' has more than one primary key", table_name),
    };

    let fields_not_skip_nor_pk = fields.iter().filter(|f| { !f.skip && !f.pk }).collect::<Vec<_>>();

    let field_setter_funcs = fields_not_skip_nor_pk.iter().map(|f| {
        let ident = f.ident.clone().unwrap();
        let ty = f.ty.clone();
        let setter_fn_name = syn::Ident::new(&format!("set_{}", &ident), ident.span());
        let query = format!("UPDATE {} SET {} = $1 WHERE {} = $2 RETURNING *", table_name, ident, pk_column_ident);
        quote! {
            pub async fn #setter_fn_name(self, pool: & ::sqlx::PgPool, #ident: &#ty) -> Result<Self, ::sqlx::Error> {
                let data = ::sqlx::query_as!(
                    Self,
                    #query,
                    #ident,
                    self.#pk_column_ident,
                )
                .fetch_one(pool)
                .await?;
                Ok(data)
            }
        }
    });
    let gen = quote! {
        impl #table_ident {
            #(#field_setter_funcs)*
        }
    };
    gen.into()

}

#[proc_macro_derive(Delete, attributes(table))]
#[proc_macro_error]
pub fn delete(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let table: TableData = TableData::from_derive_input(&derive_input).unwrap();
    let table_ident = table.ident;

    let table_name = table.name.clone().unwrap_or_else(|| table_ident.to_string().to_ascii_lowercase());

    let fields = match table.data {
        ast::Data::Struct(ref s) => s,
        _ => unreachable!()
    };

    let fields_with_pk = fields.iter().filter(|f| { f.pk }).collect::<Vec<_>>();
    
    let pk_column_ident= match fields_with_pk.len() {
        0 => abort!(derive_input.span(),"Table '{}' has no primary key", table_name),
        1 => fields_with_pk.first().unwrap().ident.as_ref().unwrap(),
        _ => abort!(derive_input.span(),"Table '{}' has more than one primary key", table_name),
    };

    let query = format!("DELETE FROM {} WHERE {} = $1 RETURNING *", table_name, pk_column_ident);

    let gen = quote! {
        impl #table_ident {
            pub async fn delete(&self, pool: & ::sqlx::PgPool) -> Result<Self, ::sqlx::Error> {
                let data = ::sqlx::query_as!(
                    Self,
                    #query,
                    self.#pk_column_ident,
                )
                .fetch_one(pool)
                .await?;
                Ok(data)
            }
        }
    };
    gen.into()
}
