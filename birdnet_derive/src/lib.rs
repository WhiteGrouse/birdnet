use proc_macro2::{TokenStream, Span, Literal};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields, Type, Ident, Attribute, TypePath, TypeArray};

#[proc_macro_derive(Codable, attributes(big_endian, little_endian, u24))]
pub fn codable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let derive_target = input.ident;

  if !input.generics.params.is_empty() {
    return syn::Error::new(Span::call_site(), "Generics type is not supported").to_compile_error().into();
  }

  let (read_fields, write_fields, read_ret) = match &input.data {
    Data::Struct(data) => match impl_for_struct(&derive_target, data, &input.attrs) {
      Ok(data) => data,
      Err(err) => return err.to_compile_error().into(),
    },
    _ => return syn::Error::new(Span::call_site(), "Only struct is supported").to_compile_error().into(),
  };

  let impl_codable = quote! {
    impl birdnet_code::Codable for #derive_target {
      fn encode<W: std::io::Write + std::io::Seek>(&self, buffer: &mut W) -> std::io::Result<()> {
        use byteorder::{WriteBytesExt, BigEndian, LittleEndian};

        #write_fields
        Ok(())
      }

      fn decode<R: std::io::Read + std::io::Seek>(buffer: &mut R) -> std::io::Result<Self> {
        use byteorder::{ReadBytesExt, BigEndian, LittleEndian};

        #read_fields
        Ok(#read_ret)
      }
    }
  };

  impl_codable.into()
}

fn impl_for_struct(derive_target: &Ident, data: &DataStruct, attrs: &[Attribute]) -> Result<(TokenStream, TokenStream, TokenStream), syn::Error> {
  match &data.fields {
    Fields::Named(fields) => {
      let mut fields_name = Vec::<Ident>::with_capacity(fields.named.len());
      let mut fields_read = Vec::<TokenStream>::with_capacity(fields.named.len());
      let mut fields_write = Vec::<TokenStream>::with_capacity(fields.named.len());
      for field in fields.named.iter() {
        let ident = field.ident.as_ref().unwrap();
        let let_ident = ident.clone();
        match &field.ty {
          Type::Path(path) => {
            let (f_read, f_write) = type_path(&let_ident, &ident.to_string(), path, &field.attrs);
            fields_read.push(f_read);
            fields_write.push(f_write);
            fields_name.push(let_ident);
          },
          Type::Array(array) => {
            let (f_read, f_write) = type_array(&let_ident, &ident.to_string(), array, &field.attrs)?;
            fields_read.push(f_read);
            fields_write.push(f_write);
            fields_name.push(let_ident);
          },
          _ => return Err(syn::Error::new(Span::call_site(), "Containing field unsupported type")),
        }
      }
      let write_fields = quote!(#(#fields_write)*);
      let read_fields = quote!(#(#fields_read)*);
      let read_ret = quote!(#derive_target { #(#fields_name),* });
      Ok((read_fields, write_fields, read_ret))
    },
    Fields::Unnamed(fields) => {
      let mut fields_name = Vec::<Ident>::with_capacity(fields.unnamed.len());
      let mut fields_read = Vec::<TokenStream>::with_capacity(fields.unnamed.len());
      let mut fields_write = Vec::<TokenStream>::with_capacity(fields.unnamed.len());
      let mut i = 0usize;
      for field in fields.unnamed.iter() {
        let ident = i.to_string();
        i += 1;
        let let_ident = Ident::new(&format!("field_{}", ident), Span::call_site());
        match &field.ty {
          Type::Path(path) => {
            let (f_read, f_write) = type_path(&let_ident, &ident, path, attrs);
            fields_read.push(f_read);
            fields_write.push(f_write);
            fields_name.push(let_ident);
          },
          Type::Array(array) => {
            let (f_read, f_write) = type_array(&let_ident, &ident, array, attrs)?;
            fields_read.push(f_read);
            fields_write.push(f_write);
            fields_name.push(let_ident);
          },
          _ => return Err(syn::Error::new(Span::call_site(), "Containing field unsupported type")),
        }
      }
      let write_fields = quote!(#(#fields_write)*);
      let read_fields = quote!(#(#fields_read)*);
      let read_ret = quote!(#derive_target (#(#fields_name),*));
      Ok((read_fields, write_fields, read_ret))
    },
    Fields::Unit => Ok((TokenStream::new(), TokenStream::new(), quote!(#derive_target))),
  }
}

fn get_rw_fn(ident: &str, is_field: bool, tyident: &Ident, attrs: &[Attribute]) -> (TokenStream, TokenStream) {
  let target = if is_field {
    if let Ok(index) = ident.parse::<usize>() {
      let ident = Literal::usize_unsuffixed(index);
      quote!(self.#ident)
    }
    else {
      let ident = Ident::new(ident, Span::call_site());
      quote!(self.#ident)
    }
  } else {
    let ident = Ident::new(ident, Span::call_site());
    quote!(#ident)
  };

  if tyident == "u8" {
    let write = quote!(buffer.write_u8(#target)?);
    let read = quote!(buffer.read_u8()?);
    (read, write)
  }
  else if tyident == "bool" {
    let write = quote!(buffer.write_u8(#target as u8)?);
    let read = quote!(buffer.read_u8()? != 0);
    (read, write)
  }
  else if tyident == "u16" || tyident == "u32" || tyident == "u64" {
    let mut be = true;
    let mut u24 = false;
    let mut processed_be = false;
    let mut processed_u24 = false;
    for attr in attrs {
      if !processed_be {
        if attr.path.is_ident("little_endian") {
          be = false;
          processed_be = true;
        }
        else if attr.path.is_ident("big_endian") {
          processed_be = true;
        }
        if processed_u24 {
          break;
        }
      }
      if !processed_u24 && attr.path.is_ident("u24") {
        u24 = true;
        processed_u24 = true;
        if processed_be {
          break;
        }
      }
    }
    let write_name = if u24 { "write_u24".to_string() } else { format!("write_{}", tyident) };
    let read_name = if u24 { "read_u24".to_string() } else { format!("read_{}", tyident) };
    let order = if be { "BigEndian" } else { "LittleEndian" };
    let write_fn = Ident::new(&write_name, Span::call_site());
    let read_fn = Ident::new(&read_name, Span::call_site());
    let order = Ident::new(&order, Span::call_site());
    let write = quote!(buffer.#write_fn::<#order>(#target)?);
    let read = quote!(buffer.#read_fn::<#order>()?);
    (read, write)
  }
  else {
    let write = quote!(#target.encode(buffer)?);
    let read = quote!(#tyident::decode(buffer)?);
    (read, write)
  }
}

fn type_path(let_ident: &Ident, ident: &str, path: &TypePath, attrs: &[Attribute]) -> (TokenStream, TokenStream) {
  let tyident = path.path.get_ident().unwrap();
  let (read_fn, write_fn) = get_rw_fn(ident, true, tyident, attrs);
  let write = quote!(#write_fn;);
  let read = quote!(let #let_ident = #read_fn;);
  (read, write)
}

fn type_array(let_ident: &Ident, ident: &str, array: &TypeArray, attrs: &[Attribute]) -> Result<(TokenStream, TokenStream), syn::Error> {
  let target = if let Ok(index) = ident.parse::<usize>() {
    let ident = Literal::usize_unsuffixed(index);
    quote!(self.#ident)
  }
  else {
    let ident = Ident::new(ident, Span::call_site());
    quote!(self.#ident)
  };
  match array.elem.as_ref() {
    Type::Path(path) => {
      let tyident = path.path.get_ident().unwrap();
      let len = &array.len;
      let (read_fn, write_fn) = get_rw_fn(&"elem", false, tyident, attrs);
      let primitive_magic = if tyident == "u8" || tyident == "u16" || tyident == "u32" || tyident == "u64" { quote!(let elem = *elem;) }
                            else { TokenStream::new() };
      let write = quote! {
        for elem in &#target[..] {
          #primitive_magic
          #write_fn;
        }
      };
      let read = quote! {
        let #let_ident = {
          use std::mem::{transmute, MaybeUninit};

          let mut #let_ident: [MaybeUninit<#tyident>; #len] = unsafe { MaybeUninit::uninit().assume_init() };
          for elem in &mut #let_ident[..] {
            *elem = MaybeUninit::new(#read_fn);
          }
          unsafe { transmute::<_, [#tyident; #len]>(#let_ident) }
        };
      };
      Ok((read, write))
    },
    _ => Err(syn::Error::new(Span::call_site(), "Containing field unsupported type")),
  }
}
