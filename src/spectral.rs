use std;
use std::cmp::min;

use arrayvec::ArrayVec;

use coefs::Coefficients;
use params::BaseParams;
use prev::PrevFrame;

#[derive(Clone)]
pub struct Spectrals(ArrayVec<[f32; 56]>);

impl Spectrals {
    pub fn new(coefs: &Coefficients, params: &BaseParams, prev: &PrevFrame) -> Spectrals {
        let mut spectral = ArrayVec::new();

        let indexes = |x: u32| {
            let k = prev.params.harmonics as f32 / params.harmonics as f32 * x as f32;
            let int = k.floor();
            let dec = k - int;

            (int as usize, dec)
        };

        let pred = (0.03 * params.harmonics as f32 - 0.05).max(0.4).min(0.7);
        let pred_scaled = pred / params.harmonics as f32;

        let sum = (1..params.harmonics+1).map(|lambda| {
            let (k, dec) = indexes(lambda);
            (1.0 - dec) * prev.spectrals.get(k).log2() +
                dec * prev.spectrals.get(k + 1).log2()
        }).fold(0.0, |s, x| s + x);

        for l in 1..params.harmonics+1 {
            let (k, dec) = indexes(l);

            spectral.push((
                coefs.get(l as usize) +
                    pred * (1.0 - dec) * prev.spectrals.get(k).log2() +
                    pred * dec * prev.spectrals.get(k + 1).log2() -
                    pred_scaled * sum
            ).exp2());
        }

        Spectrals(spectral)
    }

    pub fn get(&self, l: usize) -> f32 {
        if l == 0 {
            1.0
        } else {
            self.0[min(l, self.0.len()) - 1]
        }
    }
}

impl std::ops::Deref for Spectrals {
    type Target = ArrayVec<[f32; 56]>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Default for Spectrals {
    fn default() -> Spectrals {
        Spectrals((0..56).map(|_| 1.0).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use descramble::{Bootstrap, descramble};
    use gain::Gains;
    use coefs::Coefficients;
    use params::BaseParams;
    use prev::{PrevFrame};

    #[test]
    fn test_spectrals() {
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
        let g = Gains::new(gain_idx, &amps, &p);
        let c = Coefficients::new(&g, &amps, &p);
        let mut prev = PrevFrame::default();
        let s = Spectrals::new(&c, &p, &prev);

        assert_eq!(s.0.len(), 16);

        assert!((s.get(0) - 1.0).abs() < 0.000001);
        assert!((s.get(1) - 0.5306769781475001).abs() < 0.000001);
        assert!((s.get(2) - 0.3535007196676076).abs() < 0.000001);
        assert!((s.get(3) - 0.9173875577243951).abs() < 0.000001);
        assert!((s.get(4) - 0.13169278935782622).abs() < 0.000001);
        assert!((s.get(5) - 4.438599873836148).abs() < 0.00001);
        assert!((s.get(6) - 0.6796441620283217).abs() < 0.000001);
        assert!((s.get(7) - 0.9439604687782126).abs() < 0.000001);
        assert!((s.get(8) - 10.646341109175768).abs() < 0.000001);
        assert!((s.get(9) - 1.3058035207362984).abs() < 0.000001);
        assert!((s.get(10) - 1.566168983101457).abs() < 0.000001);
        assert!((s.get(11) - 8.325591838823936).abs() < 0.00001);
        assert!((s.get(12) - 1.5943520720633444).abs() < 0.000001);
        assert!((s.get(13) - 1.556318849691248).abs() < 0.000001);
        assert!((s.get(14) - 15.882295806333524).abs() < 0.00001);
        assert!((s.get(15) - 11.386537444591385).abs() < 0.00001);
        assert!((s.get(16) - 12.498792573794846).abs() < 0.00001);
        assert!((s.get(17) - 12.498792573794846).abs() < 0.00001);
        assert!((s.get(30) - 12.498792573794846).abs() < 0.00001);
        assert!((s.get(56) - 12.498792573794846).abs() < 0.00001);

        prev.spectrals = s;
        let s = Spectrals::new(&c, &p, &prev);

        assert!((s.get(1) - 0.18026009141598204).abs() < 0.000001);
        assert!((s.get(2) - 0.09466759399487303).abs() < 0.000001);
        assert!((s.get(3) - 0.546543586730317).abs() < 0.000001);
        assert!((s.get(4) - 0.11240949805284323).abs() < 0.000001);
        assert!((s.get(5) - 2.664303862945382).abs() < 0.000001);
        assert!((s.get(6) - 0.7356498197494585).abs() < 0.000001);
        assert!((s.get(7) - 0.6722918529743475).abs() < 0.000001);
        assert!((s.get(8) - 15.748045510026174).abs() < 0.000001);
        assert!((s.get(9) - 2.010522620838216).abs() < 0.000001);
        assert!((s.get(10) - 2.411402725277654).abs() < 0.000001);
        assert!((s.get(11) - 12.818766727159018).abs() < 0.00001);
        assert!((s.get(12) - 2.4547957296486485).abs() < 0.000001);
        assert!((s.get(13) - 2.396236648815911).abs() < 0.000001);
        assert!((s.get(14) - 24.45369037714974).abs() < 0.0001);
        assert!((s.get(15) - 17.53165062111628).abs() < 0.0001);
        assert!((s.get(16) - 19.244170201509185).abs() < 0.0001);
    }
}
