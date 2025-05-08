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
    let print_bytes_count = flag_bool(c!("c"), false, c!("Print the byte counts"));
    let print_chars_count = flag_bool(c!("m"), false, c!("Print the character counts"));
    let print_lines_count = flag_bool(c!("l"), false, c!("Print the newline counts"));
    let print_words_count = flag_bool(c!("w"), false, c!("Print the word counts"));
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
        fprintf(stderr, c!("ERROR: no input is provided\n"));
        return 1;
    }

    let mut sb: String_Builder = zeroed();

    if !read_entire_file(input_path, &mut sb) {
        return 1;
    }

    let chars = sb.count;
    let bytes = sb.count;

    let mut lines: usize = 0;
    let mut words: usize = 0;

    get_file_stats(&mut sb, &mut lines, &mut words);

    if *print_bytes_count {
        printf(c!("%zu %s\n"), bytes, input_path);
    } else if *print_chars_count {
        printf(c!("%zu %s\n"), chars, input_path);
    } else if *print_lines_count {
        printf(c!("%zu %s\n"), lines, input_path);
    } else if *print_words_count {
        printf(c!("%zu %s\n"), words, input_path);
    } else {
        printf(c!("%zu %zu %zu %s\n"), lines, words, bytes, input_path);
    }

    free(sb.items);

    0
}

unsafe fn get_file_stats(
    sb: *mut String_Builder,
    lines: *mut usize,
    words: *mut usize,
) {
    let mut lc: usize = 0;
    let mut wc: usize = 0;
    let mut in_word = false;

    for i in 0..(*sb).count {
        let c = *(*sb).items.add(i);

        if c == b'\n' as c_char {
            lc += 1;
        }

        if is_ascii_graphic(c) {
            if !in_word {
                in_word = true;
                wc += 1;
            }
        } else if c == b' ' as c_char || c == b'\n' as c_char || c == b'\t' as c_char {
            in_word = false;
        }
    }

    *lines = lc;
    *words = wc;
}


fn is_ascii_graphic(c: c_char) -> bool {
    c >= b'!' as c_char && c <= b'~' as c_char
}
