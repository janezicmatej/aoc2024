use std::str::{pattern::Pattern, FromStr};

pub fn to_vec<T, P>(s: &str, pat: P) -> Vec<T>
where
    T: FromStr,
    P: Pattern,
{
    s.split(pat).filter_map(|x| x.parse().ok()).collect()
}

pub fn to_vec_map<T, U, P>(s: &str, pat: P, func: impl FnMut(T) -> U) -> Vec<U>
where
    T: FromStr,
    P: Pattern,
{
    s.split(pat)
        .filter_map(|x| x.parse().ok())
        .map(func)
        .collect()
}
