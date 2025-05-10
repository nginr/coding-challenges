#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]
#![allow(unused_mut)]

#[macro_use]
#[path = "../crates/nob.rs"]
pub mod nob;
#[macro_use]
#[path = "../crates/flag.rs"]
pub mod flag;
#[macro_use]
#[path = "../crates/crust.rs"]
pub mod crust;

use core::ffi::*;
use core::mem::zeroed;
use core::ptr;

use crust::libc::*;
use flag::*;
use nob::*;

#[repr(i8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum JsonChar {
    OpenBrace = b'{' as i8,
    CloseBrace = b'}' as i8,
    OpenBracket = b'[' as i8,
    CloseBracket = b']' as i8,
    Colon = b':' as i8,
    Comma = b',' as i8,
    Quote = b'"' as i8,
}

impl JsonChar {
    fn from_i8(byte: i8) -> Option<Self> {
        match byte as u8 {
            b'{' => Some(JsonChar::OpenBrace),
            b'}' => Some(JsonChar::CloseBrace),
            b'[' => Some(JsonChar::OpenBracket),
            b']' => Some(JsonChar::CloseBracket),
            b':' => Some(JsonChar::Colon),
            b',' => Some(JsonChar::Comma),
            b'"' => Some(JsonChar::Quote),
            _ => None,
        }
    }
}

pub unsafe fn usage() {
    fprintf(
        stderr,
        c!("Usage: %s [OPTIONS] <input.json>\n"),
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
    let res = strcasecmp(ext, c!(".json"));

    if ext.is_null() || res != 0 {
        usage();
        fprintf(stderr, c!("ERROR: Input must be a valid JSON file\n"));
        return 1;
    }

    let mut sb: String_Builder = zeroed();

    if !read_entire_file(input_path, &mut sb) {
        return 1;
    }

    if sb.items.is_null() || sb.count == 0 {
        if !sb.items.is_null() {
            free(sb.items);
        }
        fprintf(stderr, c!("ERROR: Invalid input file: can not be empty\n"));
        return 1;
    }

    if !json_object(&mut sb) {
        if !sb.items.is_null() {
            free(sb.items);
        }
        return 1;
    }
    printf(c!("Json file: %s is VALID\n"), input_path);

    free(sb.items);
    0
}

unsafe fn json_object(sb: *mut String_Builder) -> bool {
    let expected = b'{' as i8;
    let mut curr = expect_char((*sb).items, expected);

    if curr.is_null() {
        return false;
    }

    let mut trailing_comma = false;

    loop {
        curr = skip_whitespace(curr);

        match JsonChar::from_i8(*curr) {
            Some(JsonChar::CloseBrace) => {
                let _ = curr.add(1);

                if trailing_comma {
                    fprintf(
                        stderr,
                        c!("ERROR: Trailing comma\n"),
                        expected as c_int,
                    );
                    return false;
                }

                return true;
            }

            Some(JsonChar::Quote) => {
                let next = parse_string(curr);
                if next.is_none() {
                    return false;
                }

                trailing_comma = false;

                curr = skip_whitespace(next.unwrap());

                let expected = b':' as i8;
                curr = expect_char(curr, expected);

                if curr.is_null() {
                    fprintf(
                        stderr,
                        c!("Unexpected End-of-File after '%c'\n"),
                        expected as c_int,
                    );
                    return false;
                }

                curr = skip_whitespace(curr);

                match JsonChar::from_i8(*curr) {
                    Some(JsonChar::Quote) => {
                        let next_val = parse_string(curr);
                        if next_val.is_none() {
                            return false;
                        }
                        curr = skip_whitespace(next_val.unwrap());
                    }
                    _ => {
                        fprintf(
                            stderr,
                            c!("Expected literal or \"value\" '%c'\n"),
                            expected as c_int,
                        );
                        return false;
                    }
                }

                match JsonChar::from_i8(*curr) {
                    Some(JsonChar::Comma) => {
                        curr = curr.add(1);
                        trailing_comma = true;
                        continue;
                    }
                    Some(JsonChar::CloseBrace) => {
                        curr = curr.add(1);
                        return true;
                    }
                    _ => {
                        fprintf(stderr, c!("ERROR: Expected ',' or '}' after pair\n"));
                        return false;
                    }
                }
            }
            _ => {
                fprintf(stderr, c!("ERROR: Expected '}' or \"key\" after '{'\n"));
                return false;
            }
        }
    }
}

unsafe fn parse_string(mut p: *const c_char) -> Option<*const c_char> {
    if *p != JsonChar::Quote as i8 {
        unreachable!();
    }

    p = p.add(1);

    while *p != 0 {
        if *p == JsonChar::Quote as i8 {
            return Some(p.add(1));
        }

        if *p == b'\\' as i8 && *p.add(1) != 0 {
            p = p.add(2);
        } else {
            p = p.add(1);
        }
    }

    fprintf(stderr, c!("Unterminated string\n"));
    None
}

unsafe fn skip_whitespace(mut p: *const c_char) -> *const c_char {
    while *p == b' ' as i8 || *p == b'\n' as i8 || *p == b'\r' as i8 || *p == b'\t' as i8 {
        p = p.add(1);
    }
    p
}

unsafe fn expect_char(p: *const c_char, expected: i8) -> *const c_char {
    let p = skip_whitespace(p);
    if *p == expected {
        // printf(c!("%c\n"), *p as c_int);
        p.add(1)
    } else {
        ptr::null()
    }
}
