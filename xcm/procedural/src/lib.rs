// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Digital Circulation.

// Digital Circulation is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Digital Circulation is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Digital Circulation.  If not.

//! Procedural macros used in XCM.

use proc_macro::TokenStream;

mod v0;
mod v1;
mod weight_info;

#[proc_macro]
pub fn impl_conversion_functions_for_multilocation_v0(input: TokenStream) -> TokenStream {
	v0::multilocation::generate_conversion_functions(input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro]
pub fn impl_conversion_functions_for_multilocation_v1(input: TokenStream) -> TokenStream {
	v1::multilocation::generate_conversion_functions(input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro_derive(XcmWeightInfoTrait)]
pub fn derive_xcm_weight_info(item: TokenStream) -> TokenStream {
	weight_info::derive(item)
}
