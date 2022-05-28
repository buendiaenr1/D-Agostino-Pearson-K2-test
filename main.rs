use statrs::distribution::{ChiSquared, Continuous};

use std::io;

/*  Transcripción y ajuste a RUST por:
        Enrique R.P. Buendia Lozada
    Mayo 2022

    Referencias:
    https://www.mathworks.com/matlabcentral/mlc-downloads/downloads/submissions/46548/versions/2/previews/Codes_data_publish/Codes/Dagostest.m/index.html
    https://en.wikipedia.org/wiki/D%27Agostino%27s_K-squared_test


*/

//
fn leer() {
    //let mut xs: Vec<f64> = vec![2.4, 2.6, 2.7, 3.2, 3.2, 3.4, 3.4, 3.5, 3.5, 3.6];

    let mut xs: Vec<f64> = Vec::new();

    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
        let record = result.expect("a CSV record");

        xs.push(record[0].parse::<f64>().unwrap());
    }
    //println!(" {:?}", xs);
    dago_pear_k2(&mut xs);
}

fn main() {
    println!("    ");
    println!("  Control estadístico, BUAP México, ");
    println!("  Mayo de 2022");
    println!("  Autor:  Dr. Enrique R.P. Buendia Lozada  ");
    println!("    ");
    leer();
}

use std::cmp::Ordering;
fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn dago_pear_k2(x: &mut Vec<f64>) -> bool {
    //x.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
    x.sort_by(cmp_f64);
    //print!("{:?}", x);

    let alpha: f64 = 0.05;
    let n = x.len();
    let s1: f64 = x.iter().sum(); // suma de los elementos de x
                                  //println!("\ns1 ::: {}\n", s1);
    let s2: f64 = x.iter().map(|a| a.powi(2)).sum::<f64>(); //suma de cada elemento al cuadrado
    let s3: f64 = x.iter().map(|a| a.powi(3)).sum::<f64>(); //suma de cada elemento al cubo
    let s4: f64 = x.iter().map(|a| a.powi(4)).sum::<f64>(); //suma de cada elemento a la 4a

    //println!("\n n: {} s1 {} s2 {} s3 {} s4 {}", n, s1, s2, s3, s4);

    let ss: f64 = s2 - (f64::powf(s1, 2.0) / n as f64);
    let v: f64 = ss / (n as f64 - 1.0);
    let k3: f64 = ((n as f64 * s3) - (3.0 * s1 * s2) + ((2.0 * f64::powf(s1, 3.0)) / n as f64))
        / ((n as f64 - 1.0) * (n as f64 - 2.0));
    let g1: f64 = k3 / f64::powf(f64::powf(v, 3.0), 0.5);
    //println!("\nss {}  v {}  k3 {}   g1{}", ss, v, k3, g1);

    let k4: f64 = ((n as f64 + 1.0)
        * ((n as f64 * s4) - (4.0 * s1 * s3) + (6.0 * (f64::powf(s1, 2.0)) * (s2 / n as f64))
            - ((3.0 * (f64::powf(s1, 4.0))) / (f64::powf(n as f64, 2.0))))
        / ((n as f64 - 1.0) * (n as f64 - 2.0) * (n as f64 - 3.0)))
        - ((3.0 * (f64::powf(ss, 2.0))) / ((n as f64 - 2.0) * (n as f64 - 3.0)));

    let g2: f64 = k4 / f64::powf(v, 2.0);
    let eg1: f64 = ((n as f64 - 2.0) * g1) / f64::powf(n as f64 * (n as f64 - 1.0), 0.5);
    //let eg2: f64 = ((n as f64 - 2.0) * (n as f64 - 3.0) * g2)
    //    / ((n as f64 + 1.0) * (n as f64 - 1.0))
    //    + ((3.0 * (n as f64 - 1.0)) / (n as f64 + 1.0));
    //println!("\nk4 {}  g2 {}   eg1 {} ", k4, g2, eg1);

    let a: f64 = eg1
        * f64::powf(
            ((n as f64 + 1.0) * (n as f64 + 3.0)) / (6.0 * (n as f64 - 2.0)),
            0.5,
        );
    let b: f64 = (3.0
        * ((f64::powf(n as f64, 2.0)) + (27.0 * n as f64) - 70.0)
        * ((n as f64 + 1.0) * (n as f64 + 3.0)))
        / ((n as f64 - 2.0) * (n as f64 + 5.0) * (n as f64 + 7.0) * (n as f64 + 9.0));
    let c: f64 = f64::powf(2.0 * (b - 1.0), 0.5) - 1.0;
    let d: f64 = f64::powf(c, 0.5);
    let e: f64 = 1.0 / f64::powf(d.ln(), 0.5);
    let f: f64 = a / f64::powf(2.0 / (c - 1.0), 0.5);
    //println!("a {}  b {}  c {}  d {} e {} f{}", a, b, c, d, e, f);
    let zg1: f64 = e * (f + f64::powf(f64::powf(f, 2.0) + 1.0, 0.5)).ln();
    let g: f64 = (24.0 * n as f64 * (n as f64 - 2.0) * (n as f64 - 3.0))
        / (f64::powf(n as f64 + 1.0, 2.0) * (n as f64 + 3.0) * (n as f64 + 5.0));
    let h: f64 = ((n as f64 - 2.0) * (n as f64 - 3.0) * g2.abs())
        / ((n as f64 + 1.0) * (n as f64 - 1.0) * f64::powf(g, 0.5));
    let j: f64 = ((6.0 * (f64::powf(n as f64, 2.0) - (5.0 * n as f64) + 2.0))
        / ((n as f64 + 7.0) * (n as f64 + 9.0)))
        * f64::powf(
            (6.0 * (n as f64 + 3.0) * (n as f64 + 5.0))
                / (n as f64 * (n as f64 - 2.0) * (n as f64 - 3.0)),
            0.5,
        );
    //println!("\nzg1 {} g {}  h {}  j {}", zg1, g, h, j);
    let k: f64 = 6.0 + ((8.0 / j) * ((2.0 / j) + f64::powf(1.0 + (4.0 / f64::powf(j, 2.0)), 0.5)));
    let l: f64 = (1.0 - (2.0 / k)) / (1.0 + h * f64::powf(2.0 / (k - 4.0), 0.5));
    let zg2: f64 =
        (1.0 - (2.0 / (9.0 * k)) - f64::powf(l, 1. / 3.0)) / f64::powf(2.0 / (9.0 * k), 0.5);
    let k2: f64 = f64::powf(zg1, 2.0) + f64::powf(zg2, 2.0); // D'Agostino-Pearson statistic
                                                             //print!("\nk {} l {} zg2 {} k2: {}", k, l, zg2, k2);
                                                             //println!("\n k2 {}", k2);
    let x2: f64 = k2;
    let df: f64 = 2.0;

    let nn = ChiSquared::new(df).unwrap();
    let prob: f64 = nn.pdf(x2) * 2.0;
    println!("\n D'Agostino-Pearson normality test\n\n K2 is distributed as Chi-squared with df=2");
    println!(" k2 {}        p {}\n", x2, prob);

    if prob >= alpha {
        println!(" distribution is normal");
        return true;
    } else {
        println!(" distribution is not normal");
        return false;
    }
}
