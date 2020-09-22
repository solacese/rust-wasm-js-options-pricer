#[allow(non_snake_case)]
use crate::bs::OptionDir;
use wide::*;

fn erf_f32x8(x: f32x8) -> f32x8 {
    let t = x.sign_bit();
    let e = x.abs();
    let n: f32x8 = f32x8::splat(0.3275911);
    let a: f32x8 = f32x8::splat(0.254829592);
    let r: f32x8 = f32x8::splat(-0.284496736);
    let i: f32x8 = f32x8::splat(1.421413741);
    let l: f32x8 = f32x8::splat(-1.453152027);
    let d: f32x8 = f32x8::splat(1.061405429);
    let u = f32x8::ONE / e.mul_add(n, f32x8::ONE);
    let eu = u * (-e * e).exp();
    let m = eu.mul_neg_add(
        u.mul_add(u.mul_add(u.mul_add(d.mul_add(u, l), i), r), a),
        f32x8::ONE,
    );
    t.blend(-m, m)
}

fn ncd_f32x8(e: f32x8) -> f32x8 {
    let v = f32x8::HALF * (f32x8::ONE + erf_f32x8(e / f32x8::SQRT_2));
    let min: f32x8 = f32x8::splat(-1.0e5);
    let max: f32x8 = f32x8::splat(1.0e5);

    let zero_mask = e.cmp_lt(min);
    let one_mask = e.cmp_gt(max);
    let v = zero_mask.blend(f32x8::ZERO, v);
    let v = one_mask.blend(f32x8::ONE, v);
    v
}

fn npd_f32x8(e: f32x8) -> f32x8 {
    const C: f32 = 0.3989422804014330;
    (0.5 * e * e).exp() * C
}

// t - spot
// n - strike
// r - volatility
// a - years_to_expiry
// i - interest
// l - dividend
pub(crate) fn call_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = 1.0 / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    // Call specific
    let o = ncd_f32x8(d1);
    let c = ncd_f32x8(d2);
    o * spot * la - c * g
}

pub(crate) fn call_delta_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let la = (-dividend_yield * years_to_expiry).exp();
    // Call specific
    let o = ncd_f32x8(d1);
    la * o
}

pub(crate) fn put_delta_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let la = (-dividend_yield * years_to_expiry).exp();
    //let ia = (-risk_free_rate * years_to_expiry).exp();
    // Call specific
    let o = ncd_f32x8(-d1);
    -la * o
}

pub(crate) fn gamma_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    -la * v / (spot * volatility * d)
}

pub(crate) fn vega_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    spot * la * v * d
}
pub(crate) fn call_theta_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    // Call specific
    let o = ncd_f32x8(d1);
    let c = ncd_f32x8(d2);
    -la * spot * v * volatility / (2.0 * d) - risk_free_rate * g * c
        + dividend_yield * spot * la * o
}

pub(crate) fn put_theta_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    let o = ncd_f32x8(-d1);
    let c = ncd_f32x8(-d2);
    -la * spot * v * volatility / (2.0 * d) + risk_free_rate * g * c
        - dividend_yield * spot * la * o
}
pub(crate) fn call_rho_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    // Call specific
    let c = ncd_f32x8(d2);
    g * years_to_expiry * c
}

/// Calculate rho for a wide set of values
pub(crate) fn put_rho_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    let c = ncd_f32x8(-d2);
    -g * years_to_expiry * c
}

pub(crate) fn put_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    //let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    // Put specific
    let o = ncd_f32x8(-d1);
    let c = ncd_f32x8(-d2);
    c * g - o * spot * la
}

/// Black Scholes single option pricing
pub(crate) fn price_f32x8(
    dir: OptionDir,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    match dir {
        OptionDir::CALL => call_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
        OptionDir::PUT => put_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
    }
}

/// Delta calculator
pub(crate) fn delta(
    option_dir: OptionDir,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    match option_dir {
        OptionDir::CALL => call_delta_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
        OptionDir::PUT => put_delta_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
    }
}

pub(crate) fn theta(
    option_dir: OptionDir,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    match option_dir {
        OptionDir::CALL => call_theta_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
        OptionDir::PUT => put_theta_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
    }
}

pub(crate) fn rho(
    option_dir: OptionDir,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    match option_dir {
        OptionDir::CALL => call_rho_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
        OptionDir::PUT => put_rho_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        ),
    }
}

pub(crate) fn implied_vol_f32x8(
    option_dir: OptionDir,
    price: f32x8,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let mut volatility = f32x8::splat(0.2);
    loop {
        let option_value = price_f32x8(
            option_dir,
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let diff = option_value - price;
        let mask = diff.abs().cmp_lt(f32x8::splat(0.001));
        if mask.all() {
            break;
        }
        let derivative = vega_f32x8(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let bump_mask = diff.cmp_gt(f32x8::ZERO);
        let bump_value = diff / derivative;
        volatility = bump_mask.blend(volatility - bump_value, volatility + bump_value);
    }
    volatility
}

pub(crate) fn implied_ir_f32x8(
    option_dir: OptionDir,
    price: f32x8,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> f32x8 {
    let mut risk_free_rate: f32x8 = 0.05.into();
    loop {
        let option_value = price_f32x8(
            option_dir,
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let derivative = rho(
            option_dir,
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );

        let diff = option_value - price;
        let mask = diff.abs().cmp_lt(0.0001.into());
        if mask.all() {
            break;
        }
        let bump_mask = diff.cmp_gt(f32x8::ZERO);
        let bump_value = diff / derivative;
        risk_free_rate = bump_mask.blend(risk_free_rate - bump_value, risk_free_rate + bump_value);
        // Extremes
        risk_free_rate = risk_free_rate
            .cmp_lt(f32x8::ZERO)
            .blend(f32x8::ZERO, risk_free_rate);
        risk_free_rate = risk_free_rate
            .cmp_gt(2.0.into())
            .blend(2.0.into(), risk_free_rate);
    }
    risk_free_rate
}

pub(crate) fn call_strike_from_delta_f32x8(
    delta: f32x8,
    spot: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    years_to_expiry: f32x8,
) -> f32x8 {
    let tsq = years_to_expiry.sqrt();
    let d1 = delta * (risk_free_rate * years_to_expiry).exp();
    let t_0 = -ncd_f32x8(d1) * volatility * tsq + (volatility * volatility) / 2.0 * years_to_expiry;
    spot * t_0.abs()
}

pub(crate) fn put_strike_from_delta_f32x8(
    delta: f32x8,
    spot: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    years_to_expiry: f32x8,
) -> f32x8 {
    let tsq = years_to_expiry.sqrt();
    let d1 = delta * (risk_free_rate * years_to_expiry).exp();
    let t_0 = ncd_f32x8(d1) * volatility * tsq + (volatility * volatility) / 2.0 * years_to_expiry;
    spot * t_0.abs()
}

pub struct Greek {
    pub pv: f32x8,
    pub delta: f32x8,
    pub theta: f32x8,
    pub gamma: f32x8,
    pub rho: f32x8,
    pub vega: f32x8,
}

pub(crate) fn call_greeks_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> Greek {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    let v = npd_f32x8(d1);
    // Call specific
    let o = ncd_f32x8(d1);
    let c = ncd_f32x8(d2);
    let pv = o * spot * la - c * g;
    let delta = la * o;
    let gamma = -la * v / (spot * volatility * d);
    let vega = spot * la * v * d;
    let theta = -la * spot * v * volatility / (2.0 * d) - risk_free_rate * g * c
        + dividend_yield * spot * la * o;
    let rho = g * years_to_expiry * c;
    Greek {
        pv,
        delta,
        theta,
        gamma,
        rho,
        vega,
    }
}

pub(crate) fn put_greeks_f32x8(
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8,
    risk_free_rate: f32x8,
    volatility: f32x8,
    dividend_yield: f32x8,
) -> Greek {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = f32x8::ONE / rd * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    //let v = npd_f32x8(d1);
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    let v = npd_f32x8(d1);

    // Put specific
    let o = ncd_f32x8(-d1);
    let c = ncd_f32x8(-d2);
    let pv = c * g - o * spot * la;
    let delta = -la * o;
    let gamma = -la * v / (spot * volatility * d);
    let vega = spot * la * v * d;
    let theta = -la * spot * v * volatility / (2.0 * d) + risk_free_rate * g * c
        - dividend_yield * spot * la * o;
    let rho = -g * years_to_expiry * c;
    Greek {
        pv,
        delta,
        theta,
        gamma,
        rho,
        vega,
    }
}

/*
NOTE: TBD

pub fn american_put(
    spot: f32,
    strike: f32,
    years_to_expiry: f32,
    risk_free_rate: f32,
    volatility: f32,
    dividend_yield: f32,
) -> f32 {
    const BINOMIAL_ITER: f32 = 16f32;

    let delta_t = years_to_expiry / BINOMIAL_ITER;
    let up = (volatility * delta_t.sqrt()).exp();
    let discount_rate = ((risk_free_rate - dividend_yield) * delta_t).exp();
    let d = 1.0 / discount_rate;
    let pu = (discount_rate - d) / (up - d);
    let mut v = vec![0.0; BINOMIAL_ITER as usize];
    for j in 0..(BINOMIAL_ITER as usize) {
        let upow = up.powf(2.0 * j as f32 - BINOMIAL_ITER as f32);
        v[j] = f32::max(0.0, strike - (spot * upow));
    }

    let p0: f32 = 1.0 - pu;
    for j in ((BINOMIAL_ITER as usize + 1) - 1..0).rev() {
        for k in 0..j {
            v[k] = (p0 * v[k] + pu * v[k + 1]) / discount_rate;
        }
    }
    v[0]
}

pub fn american_put_simd(
    spot: f32,
    strike: f32,
    years_to_expiry: f32,
    risk_free_rate: f32,
    volatility: f32,
    dividend_yield: f32,
) -> f32 {
    const BINOMIAL_ITER: f32 = 16f32;

    let delta_t = years_to_expiry / BINOMIAL_ITER;
    let up = (volatility * delta_t.sqrt()).exp();
    let discount_rate = ((risk_free_rate - dividend_yield) * delta_t).exp();
    let d = 1.0 / discount_rate;
    let pu = (discount_rate - d) / (up - d);
    let mut v = vec![0.0; BINOMIAL_ITER as usize];
    for j in 0..(BINOMIAL_ITER as usize) {
        let upow = up.powf(2.0 * j as f32 - BINOMIAL_ITER as f32);
        v[j] = f32::max(0.0, strike - (spot * upow));
    }

    let p0: f32 = 1.0 - pu;
    for j in ((BINOMIAL_ITER as usize + 1) - 1..0).rev() {
        for k in 0..j {
            v[k] = (p0 * v[k] + pu * v[k + 1]) / discount_rate;
        }
    }
    v[0]
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bs_single::*;
    use bytemuck::cast;
    #[test]
    fn erf_check() {
        for i in (-100..100).step_by(1) {
            let expected = erf(i as f32 / 100.0);
            let actual: [f32; 8] = cast(erf_f32x8((i as f32 / 100.0).into()));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn npd_check() {
        for i in (-100..100).step_by(1) {
            let expected = npd(i as f32 / 100.0);
            let actual: [f32; 8] = cast(npd_f32x8((i as f32 / 100.0).into()));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn ncd_check() {
        for i in (-100..100).step_by(1) {
            let expected = ncd(i as f32 / 100.0);
            let actual: [f32; 8] = cast(ncd_f32x8((i as f32 / 100.0).into()));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn ncd_perf() {
        let now = std::time::Instant::now();
        const F: f32 = 0.2;
        for _ in 0..(10_000_000 / 8) {
            ncd_f32x8(F.into());
        }
        let duration = now.elapsed().as_millis();
        println!("Time take {}ms", duration);
    }

    #[test]
    fn call_check() {
        for i in (50..200).step_by(1) {
            let spot = 60.0;
            let strike = i as f32;
            let years_to_expiry = (i as f32 / 4.0) / 252.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01 + (i as f32 / 100.0);

            // Basic call/put test
            let expected = call(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(call_f32x8(
                f32x8::splat(spot),
                f32x8::splat(strike),
                f32x8::splat(years_to_expiry),
                f32x8::splat(risk_free_rate),
                f32x8::splat(volatility),
                f32x8::splat(dividend_yield),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn put_check() {
        for i in (50..160).step_by(1) {
            let spot = 150.0;
            let strike = i as f32;
            let years_to_expiry = (i as f32 / 4.0) / 252.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01 + (i as f32 / 100.0);

            // Basic call/put test
            let expected = put(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(put_f32x8(
                f32x8::splat(spot),
                f32x8::splat(strike),
                f32x8::splat(years_to_expiry),
                f32x8::splat(risk_free_rate),
                f32x8::splat(volatility),
                f32x8::splat(dividend_yield),
            ));
            assert!((actual[0] - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn vega_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = vega(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(vega_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 100.0);
        }
    }

    #[test]
    fn gamma_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = gamma(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(gamma_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }
    #[test]
    fn rho_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = put_rho(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(put_rho_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);

            let expected = call_rho(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(call_rho_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn delta_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = put_delta(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(put_delta_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);

            let expected = call_delta(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(call_delta_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn theta_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = put_theta(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(put_theta_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);

            let expected = call_theta(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );

            let actual: [f32; 8] = cast(call_theta_f32x8(
                spot.into(),
                strike.into(),
                years_to_expiry.into(),
                risk_free_rate.into(),
                volatility.into(),
                dividend_yield.into(),
            ));
            assert!((actual[0] - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn check_iv_from_price_f32x8() {
        let spot = 100.0;
        let strike = 100.0;
        let years_to_expiry = 24.0 / 252.0;
        let risk_free_rate = 0.02;
        let volatility = 0.18;
        let dividend_yield = 0.00;

        // Basic call/put test
        let call_s = call(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let v: [f32; 8] = cast(implied_vol_f32x8(
            OptionDir::CALL,
            call_s.into(),
            spot.into(),
            strike.into(),
            years_to_expiry.into(),
            risk_free_rate.into(),
            dividend_yield.into(),
        ));
        assert!((v[0] - volatility).abs() < 0.001);
    }

    #[test]
    fn check_ir_from_price_f32x8() {
        let spot = 100.0;
        let strike = 100.0;
        let years_to_expiry = 24.0 / 252.0;
        let risk_free_rate = 0.02;
        let volatility = 0.18;
        let dividend_yield = 0.00;

        // Basic call/put test
        let call_s = call(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let v: [f32; 8] = cast(implied_ir_f32x8(
            OptionDir::CALL,
            call_s.into(),
            spot.into(),
            strike.into(),
            years_to_expiry.into(),
            volatility.into(),
            dividend_yield.into(),
        ));
        assert!((v[0] - risk_free_rate).abs() < 0.001);
    }

    #[test]
    fn put_perf() {
        let spot = 150.0.into();
        let strike = 156.0.into();
        let years_to_expiry = (24.0 / 252.0).into();
        let risk_free_rate = 0.02.into();
        let volatility = 0.2.into();
        let dividend_yield = 0.01.into();

        let now = std::time::Instant::now();

        for _ in 0..10_000_000 / 8 {
            // Basic call/put test
            let _ = put_f32x8(
                spot,
                strike,
                years_to_expiry,
                risk_free_rate,
                volatility,
                dividend_yield,
            );
        }
        let duration = now.elapsed().as_millis();
        println!("Time take {}ms", duration);
    }

    #[test]
    fn cdf_perf() {
        let now = std::time::Instant::now();
        const F: f32 = 0.2;
        for _ in 0..10_000_000 / 8 {
            ncd_f32x8(F.into());
        }
        let duration = now.elapsed().as_millis();
        println!("Time take {}ms", duration);
    }
}
/*
    #[test]
    fn basic_tests() {
        let spot = 100.0;
        let strike = 100.0;
        let years_to_expiry = 24.0 / 252.0;
        let risk_free_rate = 0.02;
        let volatility = 0.2;
        let dividend_yield = 0.00;

        // Basic call/put test
        let call_s = call(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let put_s = put(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        println!("put/call {}  {}", call_s, put_s);

        assert!((call_s - 2.5559196).abs() < 0.00001);
        assert!((put_s - 2.3656273).abs() < 0.00001);

        // With dividends
        let dividend_yield = 0.05;
        let call_s = call(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let put_s = put(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        println!("put/call {}  {}", call_s, put_s);
        assert!((call_s - 2.3140182).abs() < 0.00001);
        assert!((put_s - 2.5987892).abs() < 0.00001);

        let call_d = call_delta(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );

        let put_d = put_delta(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        println!("delta {}  {}", call_d, put_d);

        let vega = vega(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );

        println!("vega {}  ", vega);

        let call_d = call_theta(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let put_d = put_theta(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        println!("theta {}  {}", call_d, put_d);

        let call_d = call_rho(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        let put_d = put_rho(
            spot,
            strike,
            years_to_expiry,
            risk_free_rate,
            volatility,
            dividend_yield,
        );
        println!("rho {}  {}", call_d, put_d);
    }

    #[test]
    fn cdf_f32() {}

    #[test]
    fn cdf_f32_single() {
        let now = std::time::Instant::now();
        let f = 0.2f32;
        let mut x = 0.0;
        for _ in 0..16000000 {
            x += cdf(f);
        }
        let duration = now.elapsed().as_millis();
        println!("Time take {}ms", duration);
        assert!(x > 1.0);
    }
}
*/
