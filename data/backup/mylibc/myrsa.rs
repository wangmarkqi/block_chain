use crate::mylibc::bindings as b;
use crate::common::tools::*;
use std::ffi::CString;
use crate::mylibc::bindings::public_key_class;
use serde::{Deserialize, Serialize};
use ring::{agreement, rand};

pub fn test() {

}

pub fn all() {
    let (pc, sc) = gen_keys();

    let iv = rand_16_char_u8().to_vec();
    dbg!(iv.clone());
    let encres = enc(pc, iv);
    dbg!(encres.clone());
    let decres = dec(sc, encres);
    dbg!(decres);
}

pub fn enc(pc: b::public_key_class, iv: Vec<u8>) -> Vec<i64> {
    let pc_box = Box::new(pc);
    let pc_ptr = Box::into_raw(pc_box);

    let len = iv.len() as u32;
    let iv2 = CString::new(iv).unwrap();
    let ivchar = iv2.as_ptr();
    let enc_ptr = unsafe { b::rsa_encrypt(ivchar, len, pc_ptr) };

    let arr: &[i64] = unsafe { std::slice::from_raw_parts(enc_ptr as *const i64, len as usize) };
    println!("{:?}", arr);
    arr.to_vec()
}

pub fn dec(sc: b::private_key_class, enc: Vec<i64>) -> Vec<u8> {
    let sc_box = Box::new(sc);
    let sc_ptr = Box::into_raw(sc_box);
    let len = enc.len() * 8;
    let newlen = len as u32;
    let enc_arr = &enc[..];
    let enc_ptr = enc_arr.as_ptr();

    let res = unsafe { b::rsa_decrypt(enc_ptr, newlen, sc_ptr) };
    let res_arr: &[u8] = unsafe { std::slice::from_raw_parts(res as *const u8, len) };

    // let dec = unsafe{CString::from_raw(res2)};
    res_arr.to_vec()
}

pub fn gen_keys() -> (b::public_key_class, b::private_key_class) {
    let pk_struct = b::public_key_class {
        modulus: 0,
        exponent: 0,
    };
    let pk_box: Box<b::public_key_class> = Box::new(pk_struct);
    let pk_ptr = Box::into_raw(pk_box);
    let sk_struct = b::private_key_class {
        modulus: 0,
        exponent: 0,
    };
    let sk_box: Box<b::private_key_class> = Box::new(sk_struct);
    let sk_ptr = Box::into_raw(sk_box);
    let dir = get_dot_env("LIBC").unwrap();

    let primetxt = format!("{}/primes.txt", dir);
    let c_to_print = CString::new(primetxt).expect("CString::new failed");
    let prime_f = c_to_print.as_ptr();
    // from raw等于释放内存
    unsafe {
        b::rsa_gen_keys(pk_ptr, sk_ptr, prime_f);
        let a = Box::from_raw(pk_ptr);
        let b = Box::from_raw(sk_ptr);
        let aa = *a;
        let bb = *b;
        (aa, bb)
    }
}

// 如果传递struct有string，一定要把str长度传递进来。
fn ptr_cstring(ptr: *mut i8, l: usize) -> String {
    let res =
        unsafe {
            // 把一个字符串指针，copy到一个slice bytes返回地址,数组应该也可以用这种方法
            let arr: &[u8] = std::slice::from_raw_parts(ptr as *const u8, l);
            CString::new(arr).expect("c string error")
        };
    let a = res.into_string().unwrap();
    a
}

