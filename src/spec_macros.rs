use proc_macro::TokenStream;
use quote::quote;
use syn;

// This macro implements from<*Spec> for Component
pub fn impl_spec_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<#name> for Position {
            fn from(val: #name) -> Self {
                Position { point: val.point }
            }
        }
    
        impl From<#name> for Renderable {
            fn from(val: #name) -> Renderable {
                Renderable {
                    glyph: val.glyph,
                    fg: val.fg,
                    bg: val.bg,
                }
            }
        }
    
        impl From<#name> for Option<CombatStats> {
            fn from(val: #name) -> Option<CombatStats> {
                val.combat_stats
            }
        }
    
        impl From<#name> for Name {
            fn from(val: #name) -> Name {
                Name { name: val.name }
            }
        }
    };
    gen.into()
}