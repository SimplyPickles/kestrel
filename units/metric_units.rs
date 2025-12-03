pub struct Unit {
    pub scalar: f64,
    pub name: &'static str,
    pub alias: &'static str,
}

macro_rules! metric_prefixes {
    ( $( $name:expr, $alias:expr, $scalar:expr );* $(;)? ) => {
        &[
            $(
                Unit {
                    name: $name,
                    alias: $alias,
                    scalar: $scalar,
                }
            ),*
        ]
    };
}

pub static PREFIXES: &[Unit] = metric_prefixes! {
    "quetta", "Q",  1e30;
    "ronna",  "R",  1e27;
    "yotta",  "Y",  1e24;
    "zetta",  "Z",  1e21;
    "exa",    "E",  1e18;
    "peta",   "P",  1e15;
    "tera",   "T",  1e12;
    "giga",   "G",   1e9;
    "mega",   "M",   1e6;
    "kilo",   "k",   1e3;
    "hecto",  "h",   1e2;
    "deca",   "da",  1e1;
    "deci",   "d",  1e-1;
    "centi",  "c",  1e-2;
    "milli",  "m",  1e-3;
    "micro",  "μ",  1e-6;
    "nano",   "n",  1e-9;
    "pico",   "p",  1e-12;
    "femto",  "f",  1e-15;
    "atto",   "a",  1e-18;
    "zepto",  "z",  1e-21;
    "yocto",  "y",  1e-24;
    "ronto",  "r",  1e-27;
    "quecto", "q",  1e-30;
};
