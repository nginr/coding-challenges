#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]
#![allow(unused_mut)]

#[macro_use]
#[path="../crates/nob.rs"] 
pub mod nob;
#[macro_use]
#[path="../crates/flag.rs"] 
pub mod flag;
#[macro_use]
#[path="../crates/crust.rs"] 
pub mod crust;

use core::alloc;
use core::ffi::*;
use core::mem::zeroed;
use core::ptr;
use core::slice;
use crust::libc::*;
use flag::*;
use nob::*;

pub unsafe fn usage() {
    fprintf(
        stderr,
        c!("Usage: %s [OPTIONS] <input.b>\n"),
        flag_program_name(),
    );
    fprintf(stderr, c!("OPTIONS:\n"));
    flag_print_options(stderr);
}

pub unsafe fn main(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    let help = flag_bool(c!("h"), false, c!("Print this help message"));

    let mut input_path: *const c_char = ptr::null_mut();

    while argc > 0 {
        if !flag_parse(argc, argv) {
            usage();
            flag_print_error(stderr);
            return 1;
        }

        argc = flag_rest_argc();
        argv = flag_rest_argv();
        if argc > 0 {
            if !input_path.is_null() {
                fprintf(
                    stderr,
                    c!("ERROR: Serveral input files is not supported yet\n"),
                );
                return 1;
            }
            input_path = shift!(argv, argc);
        }
    }

    if *help {
        usage();
        return 0;
    }

    if input_path.is_null() {
        usage();
        fprintf(stderr, c!("ERROR: No input is provided\n"));
        return 1;
    }

    let ext = strrchr(input_path, '.' as c_int);
    printf(c!("%s -> %s\n"), input_path, ext.add(1));

    let mut sb: String_Builder = zeroed();

    if !read_entire_file(input_path, &mut sb) {
        return 1;
    }

    if !sb.items.is_null() && sb.count == 0 {
        free(sb.items);
        fprintf(stderr, c!("ERROR: Input file can not be empty.\n"));
        return 1;
    }

    

    free(sb.items);
    0
}

