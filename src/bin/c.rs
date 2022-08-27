#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let a = parse_line!(f64, f64);
    let b = parse_line!(f64, f64);
    let c = parse_line!(f64, f64);
    let d = parse_line!(f64, f64);
    let mut flg = true;
    let ab = (b.0 - a.0, b.1 - a.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let cd = (d.0 - c.0, d.1 - c.1);
    let da = (a.0 - d.0, a.1 - d.1);

    if flg {
        flg = angle(ab, bc);
    }
    if flg {
        flg = angle(bc, cd);
    }
    if flg {
        flg = angle(cd, da);
    }
    if flg {
        flg = angle(da, ab);
    }

    let ans = if flg { "Yes" } else { "No" };
    println!("{}", ans);
}

fn angle(a: (f64, f64), b: (f64, f64)) -> bool {
    a.0 * b.1 - a.1 * b.0 > 0_f64
}
