use std::f32::consts::PI;

use descramble::QuantizedAmplitudes;
use params::BaseParams;
use allocs::allocs;

pub struct Gains([f32; 6]);

impl Gains {
    pub fn new(gain_idx: usize, amps: &QuantizedAmplitudes, params: &BaseParams) -> Gains {
        let mut gains = [0.0; 6];

        let (_, alloc) = allocs(params.harmonics);
        let steps = steps(params.harmonics);

        gains[0] = GAIN[gain_idx];

        for m in 2..7 {
            let bits = alloc[m + 1 - 3] as i32;
            let step = steps[m + 1 - 3];

            gains[m - 1] = if bits == 0 {
                0.0
            } else {
                step * (amps.get(m + 1) as f32 - (2.0f32).powi(bits - 1) + 0.5)
            };
        }

        Gains(gains)
    }

    pub fn idct(&self, i: usize) -> f32 {
        assert!(i >= 1 && i <= 6);

        self.0[0] + (2..7).map(|m| {
            2.0 * self.0[m - 1] *
                (PI * (m as f32 - 1.0) * (i as f32 - 0.5) / 6.0).cos()
        }).fold(0.0, |s, x| s + x)
    }
}

fn steps(harmonics: u32) -> &'static [f32; 5] {
    &STEPS[harmonics as usize - 9]
}

// STEP[l][m] is del_m for l
static STEPS: [[f32; 5]; 48] = [
    [0.003100, 0.004020, 0.003360, 0.002900, 0.002640],
    [0.006200, 0.004020, 0.006720, 0.005800, 0.005280],
    [0.012400, 0.008040, 0.006720, 0.011600, 0.010560],
    [0.012400, 0.016080, 0.013440, 0.011600, 0.010560],
    [0.024800, 0.016080, 0.013440, 0.021750, 0.019800],
    [0.024800, 0.030150, 0.025200, 0.021750, 0.019800],
    [0.024800, 0.030150, 0.025200, 0.021750, 0.036960],
    [0.046500, 0.030150, 0.025200, 0.040600, 0.036960],
    [0.046500, 0.030150, 0.047040, 0.040600, 0.036960],
    [0.046500, 0.056280, 0.047040, 0.040600, 0.036960],
    [0.046500, 0.056280, 0.047040, 0.058000, 0.052800],
    [0.046500, 0.056280, 0.047040, 0.058000, 0.052800],
    [0.086800, 0.056280, 0.047040, 0.058000, 0.052800],
    [0.086800, 0.056280, 0.067200, 0.058000, 0.052800],
    [0.086800, 0.080400, 0.067200, 0.058000, 0.052800],
    [0.086800, 0.080400, 0.067200, 0.058000, 0.052800],
    [0.086800, 0.080400, 0.067200, 0.058000, 0.085800],
    [0.086800, 0.080400, 0.067200, 0.094250, 0.085800],
    [0.086800, 0.080400, 0.067200, 0.094250, 0.085800],
    [0.124000, 0.080400, 0.067200, 0.094250, 0.085800],
    [0.124000, 0.080400, 0.067200, 0.094250, 0.085800],
    [0.124000, 0.080400, 0.067200, 0.094250, 0.085800],
    [0.124000, 0.080400, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.080400, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.085800],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.094250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.124000, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.109200, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.142800, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.142800, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.142800, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.142800, 0.123250, 0.112200],
    [0.201500, 0.130650, 0.142800, 0.123250, 0.112200],
];

// G_1[b_2]
const GAIN: [f32; 64] = [
    -2.842205,
    -2.694235,
    -2.558260,
    -2.382850,
    -2.221042,
    -2.095574,
    -1.980845,
    -1.836058,
    -1.645556,
    -1.417658,
    -1.261301,
    -1.125631,
    -0.958207,
    -0.781591,
    -0.555837,
    -0.346976,
    -0.147249,
    0.027755,
    0.211495,
    0.388380,
    0.552873,
    0.737223,
    0.932197,
    1.139032,
    1.320955,
    1.483433,
    1.648297,
    1.801447,
    1.942731,
    2.118613,
    2.321486,
    2.504443,
    2.653909,
    2.780654,
    2.925355,
    3.076390,
    3.220825,
    3.402869,
    3.585096,
    3.784606,
    3.955521,
    4.155636,
    4.314009,
    4.444150,
    4.577542,
    4.735552,
    4.909493,
    5.085264,
    5.254767,
    5.411894,
    5.568094,
    5.738523,
    5.919215,
    6.087701,
    6.280685,
    6.464201,
    6.647736,
    6.834672,
    7.022583,
    7.211777,
    7.471016,
    7.738948,
    8.124863,
    8.69582,
];

#[cfg(test)]
mod tests {
    use super::*;
    use descramble::{Bootstrap, descramble};
    use params::BaseParams;

    #[test]
    fn test_gains_9() {
        let chunks = [
            0b000000010010,
            0b110011001100,
            0b111000111000,
            0b111111111111,
            0b11010110101,
            0b00101111010,
            0b01110111011,
            0b00001000,
        ];

        let b = Bootstrap::new(&chunks);
        let p = BaseParams::new(b.unwrap_period());
        let (amps, _, gain_idx) = descramble(&chunks, &p);

        assert_eq!(p.harmonics, 9);
        assert_eq!(gain_idx, 21);

        assert_eq!(amps.get(3), 443);
        assert_eq!(amps.get(4), 253);
        assert_eq!(amps.get(5), 344);
        assert_eq!(amps.get(6), 343);
        assert_eq!(amps.get(7), 159);
        assert_eq!(amps.get(8), 182);
        assert_eq!(amps.get(9), 60);
        assert_eq!(amps.get(10), 114);

        let g = Gains::new(gain_idx, &amps, &p);

        assert!((g.0[0] - 0.737223).abs() < 0.000001);
        assert!((g.0[1] - -0.21235).abs() < 0.000001);
        assert!((g.0[2] - -0.01005).abs() < 0.000001);
        assert!((g.0[3] - 0.29736).abs() < 0.000001);
        assert!((g.0[4] - 0.25375).abs() < 0.000001);
        assert!((g.0[5] - -0.25476).abs() < 0.000001);

        assert!((g.idct(1) - 0.8519942560055926).abs() < 0.000001);
        assert!((g.idct(2) - -0.13083074772702047).abs() < 0.000001);
        assert!((g.idct(3) - -0.014229409757043066).abs() < 0.000001);
        assert!((g.idct(4) - 2.0309896309891773).abs() < 0.000001);
        assert!((g.idct(5) - 0.5902767477270205).abs() < 0.000001);
        assert!((g.idct(6) - 1.0951375227622724).abs() < 0.000001);
    }

    #[test]
    fn test_gains_16() {
        let chunks = [
            0b001000010010,
            0b110011001100,
            0b111000111000,
            0b111111111111,
            0b10100110101,
            0b00101111010,
            0b01110111011,
            0b00001000,
        ];

        let b = Bootstrap::new(&chunks);
        let p = BaseParams::new(b.unwrap_period());
        let (amps, _, gain_idx) = descramble(&chunks, &p);

        assert_eq!(p.harmonics, 16);
        assert_eq!(p.bands, 6);
        assert_eq!(gain_idx, 21);

        assert_eq!(amps.get(3), 6);
        assert_eq!(amps.get(4), 34);
        assert_eq!(amps.get(5), 27);
        assert_eq!(amps.get(6), 25);
        assert_eq!(amps.get(7), 15);
        assert_eq!(amps.get(8), 36);
        assert_eq!(amps.get(9), 53);
        assert_eq!(amps.get(10), 23);
        assert_eq!(amps.get(11), 13);
        assert_eq!(amps.get(12), 14);
        assert_eq!(amps.get(13), 7);
        assert_eq!(amps.get(14), 7);
        assert_eq!(amps.get(15), 6);
        assert_eq!(amps.get(16), 4);
        assert_eq!(amps.get(17), 2);

        let g = Gains::new(gain_idx, &amps, &p);

        assert!((g.0[0] - 0.737223).abs() < 0.000001);
        assert!((g.0[1] - -1.18575).abs() < 0.000001);
        assert!((g.0[2] - 0.075375).abs() < 0.000001);
        assert!((g.0[3] - -0.1134).abs() < 0.000001);
        assert!((g.0[4] - 0.3857).abs() < 0.000001);
        assert!((g.0[5] - -0.01848).abs() < 0.000001);

        assert!((g.idct(1) - -1.2071545373041197).abs() < 0.000001);
        assert!((g.idct(2) - -1.524574246978134).abs() < 0.000001);
        assert!((g.idct(3) - 0.5032515043523329).abs() < 0.000001);
        assert!((g.idct(4) - 1.481487836406659).abs() < 0.000001);
        assert!((g.idct(5) - 1.456220246978134).abs() < 0.000001);
        assert!((g.idct(6) - 3.7141071965451276).abs() < 0.000001);
    }
}
