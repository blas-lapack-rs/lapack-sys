use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;

type Args = syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>;
type Call = syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>;

#[proc_macro_attribute]
pub fn lapack(_attr: TokenStream, func: TokenStream) -> TokenStream {
    lapack2(syn::parse(func).unwrap()).into()
}

/// TokenStream2-based main routine
fn lapack2(func: TokenStream2) -> TokenStream2 {
    let f = parse_foreign_fn(&func);
    let wrap = wrap(&f);
    quote! {
        #func
        #wrap
    }
}

/// extern "C" { fn dgetrs_(...); } -> fn dgetrs_(...);
fn parse_foreign_fn(func: &TokenStream2) -> syn::ForeignItemFn {
    let func = if let Some(func) = func.clone().into_iter().skip(2 /* 'extern', 'C' */).next() {
        if let TokenTree::Group(group) = func {
            group.stream()
        } else {
            unreachable!("#[lapack] attribute must be put to `extern \"C\"` block")
        }
    } else {
        unreachable!("#[lapack] attribute must be put to `extern \"C\"` block")
    };
    syn::parse2(func).unwrap()
}

/// Generate token stream of wrapped function
fn wrap(f: &syn::ForeignItemFn) -> TokenStream2 {
    // like dgetrs_
    let lapack_sys_name = &f.sig.ident;
    // like dgetrs
    let lapack_name = lapack_sys_name
        .to_string()
        .trim_end_matches('_')
        .to_string();
    let lapack_name = syn::Ident::new(&lapack_name, Span::call_site());
    let input = signature_input(&f.sig.inputs);
    let call = call(&f.sig.inputs);
    let output = &f.sig.output;
    quote! {
        pub unsafe fn #lapack_name ( #input ) #output {
            #lapack_sys_name ( #call )
        }
    }
}

enum ArgType {
    /// `T`
    Value(String),
    /// `*const T`
    ConstPtr(String),
    /// `*mut T`
    MutPtr(String),
}

impl From<syn::TypePtr> for ArgType {
    fn from(ptr_ty: syn::TypePtr) -> Self {
        match &*ptr_ty.elem {
            syn::Type::Path(path) => {
                let path = quote! { #path }.to_string();
                match ptr_ty.mutability {
                    Some(_) => ArgType::MutPtr(path),
                    None => ArgType::ConstPtr(path),
                }
            }
            _ => unimplemented!("Pointer for non-path is not supported yet"),
        }
    }
}

impl From<syn::TypePath> for ArgType {
    fn from(path: syn::TypePath) -> Self {
        ArgType::Value(quote! { #path }.to_string())
    }
}

/// Parse type ascription pattern `a: *mut f64` into ("a", "f64")
fn parse_input(pat: &syn::PatType) -> (String, ArgType) {
    let name = match &*pat.pat {
        syn::Pat::Ident(ident) => ident.ident.to_string(),
        _ => unreachable!(),
    };
    let arg_type = match &*pat.ty {
        syn::Type::Ptr(ptr_ty) => ptr_ty.clone().into(),
        syn::Type::Path(path) => path.clone().into(),
        _ => unimplemented!("Only Path and Pointer are supported yet"),
    };
    (name, arg_type)
}

fn is_const_ref(name: &str) -> bool {
    match name.to_lowercase().as_str() {
        "n" | "m" | "lda" | "ldb" => true,
        _ => false,
    }
}

fn is_mut_ref(name: &str) -> bool {
    match name.to_lowercase().as_str() {
        "info" => true,
        _ => false,
    }
}

/// Convert pointer-based raw-LAPACK API into value and reference based API
fn signature_input(args: &Args) -> Args {
    args.iter()
        .cloned()
        .map(|mut arg| {
            match &mut arg {
                syn::FnArg::Typed(arg) => {
                    let (name, arg_type) = parse_input(&arg);
                    let new_type = match arg_type {
                        ArgType::MutPtr(ty) => match name {
                            name if is_mut_ref(&name) => format!("&mut {}", ty),
                            _ => format!("&mut [{}]", ty),
                        },
                        ArgType::ConstPtr(ty) => match name {
                            name if is_const_ref(&name) => format!("&{}", ty),
                            _ => format!("&[{}]", ty),
                        },
                        ArgType::Value(ty) => ty,
                    };
                    *arg.ty = syn::parse_str(&new_type).unwrap();
                }
                _ => unreachable!("LAPACK raw API does not contains non-typed argument"),
            }
            arg
        })
        .collect()
}

fn call(args: &Args) -> Call {
    args.iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => {
                let (name, arg_type) = parse_input(arg);
                let expr = match arg_type {
                    ArgType::MutPtr(_) => match name {
                        name if is_mut_ref(&name) => name,
                        _ => format!("{}.as_mut_ptr()", name),
                    },
                    ArgType::ConstPtr(_) => match name {
                        name if is_const_ref(&name) => name,
                        _ => format!("{}.as_ptr()", name),
                    },
                    ArgType::Value(_) => name,
                };
                syn::parse_str::<syn::Expr>(&expr).unwrap()
            }
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_input() {
        let dgetrs = r#"
        pub fn dgetrs_(
            trans: *const c_char,
            n: *const c_int,
            nrhs: *const c_int,
            A: *const f64,
            lda: *const c_int,
            ipiv: *const c_int,
            B: *mut f64,
            ldb: *const c_int,
            info: *mut c_int,
        );
        "#;
        let f: syn::ForeignItemFn = syn::parse_str(dgetrs).unwrap();
        let result = super::signature_input(&f.sig.inputs);
        let result_str = quote! { #result }.to_string();
        let answer: TokenStream2 = syn::parse_str(
            r#"
            trans: u8,
            n: i32,
            nrhs: i32,
            A: &[f64],
            lda: i32,
            ipiv: &[i32],
            B: &mut [f64],
            ldb: i32,
            info: &mut i32
            "#,
        )
        .unwrap();
        assert_eq!(result_str, answer.to_string());
    }

    #[test]
    fn call() {
        let dgetrs = r#"
        pub fn dgetrs_(
            trans: *const c_char,
            n: *const c_int,
            nrhs: *const c_int,
            A: *const f64,
            lda: *const c_int,
            ipiv: *const c_int,
            B: *mut f64,
            ldb: *const c_int,
            info: *mut c_int,
        );
        "#;
        let f: syn::ForeignItemFn = syn::parse_str(dgetrs).unwrap();
        let result = super::call(&f.sig.inputs);
        let result_str = quote! { #result }.to_string();
        let answer: TokenStream2 = syn::parse_str(
            r#"
            &(trans as c_char),
            &n,
            &nrhs,
            A.as_ptr(),
            &lda,
            ipiv.as_ptr(),
            B.as_mut_ptr(),
            &ldb,
            info
            "#,
        )
        .unwrap();
        assert_eq!(result_str, answer.to_string());
    }

    #[test]
    fn wrap_dgetrs() {
        let dgetrs = r#"
        pub fn dgetrs_(
            trans: *const c_char,
            n: *const c_int,
            nrhs: *const c_int,
            A: *const f64,
            lda: *const c_int,
            ipiv: *const c_int,
            B: *mut f64,
            ldb: *const c_int,
            info: *mut c_int,
        );
        "#;
        let wrapped = super::wrap(&syn::parse_str(dgetrs).unwrap());
        let expected = r#"
        pub unsafe fn dgetrs(
            trans: u8,
            n: i32,
            nrhs: i32,
            A: &[f64],
            lda: i32,
            ipiv: &[i32],
            B: &mut [f64],
            ldb: i32,
            info: &mut i32
        ) {
            dgetrs_(
                &(trans as c_char),
                &n,
                &nrhs,
                A.as_ptr(),
                &lda,
                ipiv.as_ptr(),
                B.as_mut_ptr(),
                &ldb,
                info
            )
        }
        "#;
        let expected: TokenStream2 = syn::parse_str(expected).unwrap();
        assert_eq!(wrapped.to_string(), expected.to_string());
    }

    /// Test for return value case
    #[test]
    fn wrap_dlange() {
        let dgetrs = r#"
        pub fn dlange_(
            norm: *const c_char,
            m: *const c_int,
            n: *const c_int,
            A: *const f64,
            lda: *const c_int,
            work: *mut f64,
        ) -> f64;
        "#;
        let wrapped = super::wrap(&syn::parse_str(dgetrs).unwrap());
        let expected = r#"
        pub unsafe fn dlange(
            norm: u8,
            m: i32,
            n: i32,
            A: &[f64],
            lda: i32,
            work: &mut [f64]
        ) -> f64 {
            dlange_(
                &(norm as c_char),
                &m,
                &n,
                A.as_ptr(),
                &lda,
                work.as_mut_ptr()
            )
        }
        "#;
        let expected: TokenStream2 = syn::parse_str(expected).unwrap();
        assert_eq!(wrapped.to_string(), expected.to_string());
    }
}
