// 最大公约数
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::common::tools::{get_dot_env, rand_16_u8, read_file_as_txt};
use rand::Rng;
use std::collections::HashMap;
use std::env;

static SM_ENC_NAME2: &'static str = "http";
// 公钥e，n，私钥d ，n
// 加密内容
// 加密时刻，公钥e，解密私钥d
pub fn save_rsa_key_files() -> Result<String, MyError> {
    let root = get_dot_env("CRTDIR");
    let pk_path = format!("{}/{}_pk", root, SM_ENC_NAME2);
    let sk_path = format!("{}/{}_sk", root, SM_ENC_NAME2);
    let (n, e, d) = gen_keys()?;
    let mut pk = HashMap::new();
    pk.insert("n", n);
    pk.insert("e", e);
    let mut sk = HashMap::new();
    sk.insert("n", n);
    sk.insert("d", d);
    let pk_str = serde_json::to_string(&pk)?;
    let sk_str = serde_json::to_string(&sk)?;
    write_file_as_txt(&pk_path, pk_str);
    write_file_as_txt(&sk_path, sk_str);
    Ok("success".to_string())
}
pub fn gen_keys() -> Result<(i64, i64, i64), MyError> {
    if let Ok((p, q)) = get_primes() {
        let n: i64 = p * q;
        let r = (p - 1) * (q - 1);
        println!("p: {} ,q: {} N: {}, r: {}", p, q, n, r);
        let mut rng = rand::thread_rng();
        let mut e;
        loop {
            e = rng.gen::<u16>() as i64;
            if gcd(e as u64, r as u64) == 1 {
                break;
            }
        }
        let d = inv(e, r);
        println!("e: {}, d: {}", e, d);
        println!("public key is(N, e): ({}, {})", n, e);
        println!("private key is(N, d): ({}, {})", n, d);
        return Ok((n, e, d));
    }
    Err(MyError::OtherError(StrError::new("no primes")))
}

pub fn rsa_enc(n: i64, e: i64, iv: Vec<u8>) -> Vec<i64> {
    let enc: Vec<i64> = iv
        .iter()
        .map(|&a| a as i64)
        .map(|a| power_mod(a, e, n))
        .collect();
    enc
}

pub fn rsa_dec(n: i64, d: i64, enc: Vec<i64>) -> Result<Vec<u8>, MyError> {
    let dec: Vec<i64> = enc.iter().map(|&a| power_mod(a, d, n)).collect();
    let mut v = vec![];
    for i in dec.iter() {
        let a = i.to_string();
        let b = a.parse::<u8>()?;
        v.push(b);
    }
    Ok(v)
}

pub fn test() -> Result<(), MyError> {
    let (n, e, d) = gen_keys()?;
    let iv = rand_16_u8();
    let iv_vec = iv.clone().to_vec();
    let enc = rsa_enc(n, e, iv_vec);
    let dec = rsa_dec(n, d, enc.clone()).unwrap();
    dbg!(iv, enc, dec);
    Ok(())
}

fn gcd(x: u64, y: u64) -> u64 {
    let remainder = x % y;
    if remainder == 0 {
        return y;
    } else {
        return gcd(y, remainder);
    }
}

fn ext_euclid(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    if y == 0 {
        return (1, 0, x);
    } else {
        while r != 0 {
            let q = old_r / r;
            let t_r = r;
            r = old_r - q * r;
            old_r = t_r;
            let t_s = s;
            s = old_s - q * s;
            old_s = t_s;
            let t_t = t;
            t = old_t - q * t;
            old_t = t_t;
        }
        return (old_s, old_t, old_r);
    }
}

fn inv(a: i64, p: i64) -> i64 {
    let (r, _, _) = ext_euclid(a, p);
    return ((r % p) + p) % p;
}

fn get_primes() -> Result<(i64, i64), MyError> {
    let path = get_dot_env("PRIMETXT");
    let txt = read_file_as_txt(&path);
    let l: Vec<&str> = txt.split("\n").collect();
    let ll: Vec<String> = l
        .iter()
        .filter(|&&a| a != "")
        .map(|&e| e.to_string())
        .collect();

    let n = ll.len();
    let a1 = rand::thread_rng().gen_range(0, n);
    let a2 = rand::thread_rng().gen_range(0, n);
    let b1 = &ll[a1];
    let b2 = &ll[a2];
    let c1 = b1.parse::<i64>().unwrap();
    let c2 = b2.parse::<i64>().unwrap();
    Ok((c1, c2))
}

fn power_mod(base: i64, mut power: i64, n: i64) -> i64 {
    let mut bits = Vec::new();

    while power != 0 {
        match power & 1 {
            1 => bits.push(true),

            0 => bits.push(false),

            _ => {}
        }
        power = power >> 1;
    }
    let mut result: i64 = 1;
    while let Some(bit) = bits.pop() {
        result = mod_multiply(result, result, n);
        if bit {
            result = mod_multiply(result, base, n);
        }
    }
    result
}

fn mod_multiply(a: i64, b: i64, n: i64) -> i64 {
    ((a as i128 * b as i128) % (n as i128)) as i64
}

pub fn rsa_load_http_keys() -> Result<(String, String), MyError> {
    let root = get_dot_env("CRTDIR");
    let pk_path = format!("{}/{}_pk", root, SM_ENC_NAME2);
    let sk_path = format!("{}/{}_sk", root, SM_ENC_NAME2);
    let pk_js = read_file_as_txt(&pk_path);
    let sk_js = read_file_as_txt(&sk_path);
    // let pk = serde_json::from_str(pk_js)?;
    // let sk = serde_json::from_str(sk_js)?;
    Ok((pk_js, sk_js))
}
