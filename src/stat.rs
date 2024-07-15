use crate::dp_vec::*;
use statrs::distribution::*;

// returns (m, n) in y = mx + n
fn linear_regression(values: &DpVec, sample_size: u32) -> (f64, f64) {
    let s_xx = values.xn_sum(2) - values.xn_sum(1).powi(2) / (sample_size as f64);
    let s_xy = values.xy_sum() - (values.xn_sum(1) * values.y_sum()) / (sample_size as f64);

    let b1 = s_xy / s_xx;
    let b0 = (values.y_sum()) / (sample_size as f64) - b1 * (values.xn_sum(1)) / (sample_size as f64);

    (b1, b0)
}

fn p_round(f: f64, precision: u32) -> f64 {
    let tpow = 10.0_f64.powi(precision.try_into().unwrap());
    (tpow * f).round() / tpow
}

fn square_residuer_linear(data_set: &DpVec) -> (f64, f64, f64) {
    let data_size = data_set.points.len() as u32;
    let (m, n) = linear_regression(data_set, data_size);

    let mut sum: f64 = 0.0;
    for (x, y) in &data_set.points {
        sum += (y - (m*x + n))*(y - (m*x + n))
    }

    (sum, m, n)
}

pub fn chow_test(data: Vec<Vec<(f64, f64)>>) -> f64 {
    chow_test_with_precision(data, 3)
}


/***
 * Returns the p value for the lower bound test
 ***/
pub fn chow_test_with_precision(data: Vec<Vec<(f64, f64)>>, precision: u32) -> f64 {
    let mut full_data: Vec<(f64, f64)> = data[0].clone();
    full_data.append(&mut data[1].clone());

    let k: f64 = 2.0; // número de variáveis independentes + intercept
    let size_1 = data[0].len() as f64;
    let size_2 = data[1].len() as f64;
    
    let data_set_1 = DpVec {
        points: data[0].clone()
    };

    let data_set_2 = DpVec {
        points: data[1].clone()
    };

    let full_data_set = DpVec {
        points: full_data
    };

    let (sr_combined, m_combined, n_combined) = square_residuer_linear(&full_data_set);
    let (sr_1, m1, n1) = square_residuer_linear(&data_set_1);
    let (sr_2, m2, n2) = square_residuer_linear(&data_set_2); 

    println!("y1 = {}*x + {}", p_round(m1, precision), p_round(n1, precision));
    println!("y2 = {}*x + {}", p_round(m2, precision), p_round(n2, precision));
    println!("yc = {}*x + {}", p_round(m_combined, precision), p_round(n_combined, precision));

    let f = ((sr_combined - (sr_1 + sr_2)) / k) / ((sr_1 + sr_2) / (size_1 + size_2 - 2.0 * k));

    let freedom_1 = k;
    let freedom_2 = size_1 + size_2 - 2.0 * k;
    println!("Creating distibution with degrees {} {}", freedom_1, freedom_2);
    let distribution = FisherSnedecor::new(freedom_1, freedom_2).unwrap();

    println!("Testing F {}", f);

    let cdf = distribution.cdf(f);

    println!("Lower bound p-value = {}", cdf);
    println!("Upper bound p-value = {}", 1.0 - cdf);

    return cdf;
}
