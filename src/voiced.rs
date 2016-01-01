use std::cmp::max;
use std::f32::consts::PI;

use collect_slice::CollectSlice;

use consts::SAMPLES;
use descramble::VoiceDecisions;
use enhance::EnhancedSpectrals;
use noise::Noise;
use params::BaseParams;
use prev::PrevFrame;
use window;

pub struct PhaseBase([f32; 56]);

impl PhaseBase {
    pub fn new(params: &BaseParams, prev: &PrevFrame) -> PhaseBase {
        let mut phase_base = [0.0; 56];
        let common = (prev.params.fundamental - params.fundamental) *
            SAMPLES as f32 / 2.0;

        (1..57).map(|l| {
            prev.phase_base.get(l) + common * l as f32
        }).collect_slice_checked(&mut phase_base[..]);

        PhaseBase(phase_base)
    }

    pub fn get(&self, l: usize) -> f32 { self.0[l - 1] }
}


impl Default for PhaseBase {
    fn default() -> PhaseBase {
        PhaseBase([0.0; 56])
    }
}

pub struct Phase([f32; 56]);

impl Phase {
    pub fn new(params: &BaseParams, voiced: &VoiceDecisions, base: &PhaseBase) -> Phase {
        let mut noise = Noise::new();
        let mut phase = [0.0; 56];

        let trans = params.harmonics as usize / 4;

        (1..trans+1).map(|l| {
            base.get(l)
        }).chain((trans+1..57).map(|l| {
            base.get(l) + voiced.unvoiced_count as f32 * (
                2.0 * PI / 53125.0 * noise.next() as f32 - PI
            ) / params.harmonics as f32
        })).collect_slice_checked(&mut phase[..]);

        Phase(phase)
    }

    pub fn get(&self, l: usize) -> f32 { self.0[l - 1] }
}

impl Default for Phase {
    fn default() -> Phase {
        Phase([0.0; 56])
    }
}

pub struct Voiced<'a, 'b, 'c, 'd> {
    prev: &'a PrevFrame,
    phase: &'b Phase,
    enhanced: &'c EnhancedSpectrals,
    voice: &'d VoiceDecisions,
    window: window::Window,
    fundamental: f32,
    end: usize,
    freq_changed: bool,
}

impl<'a, 'b, 'c, 'd> Voiced<'a, 'b, 'c, 'd> {
    pub fn new(params: &BaseParams, prev: &'a PrevFrame, phase: &'b Phase,
               enhanced: &'c EnhancedSpectrals, voice: &'d VoiceDecisions)
        -> Voiced<'a, 'b, 'c, 'd>
    {
        let freq_diff = (params.fundamental - prev.params.fundamental).abs();

        Voiced {
            prev: prev,
            phase: phase,
            enhanced: enhanced,
            voice: voice,
            window: window::synthesis_full(),
            fundamental: params.fundamental,
            end: max(params.harmonics, prev.params.harmonics) as usize + 1,
            freq_changed: freq_diff >= 0.1 * params.fundamental,
        }
    }

    fn sig_cur(&self, l: usize, n: isize) -> f32 {
        self.window.get(n - SAMPLES as isize) * self.enhanced.get(l) * (
            self.fundamental * (n - SAMPLES as isize) as f32 * l as f32 +
                self.phase.get(l)
        ).cos()
    }

    fn sig_prev(&self, l: usize, n: isize) -> f32 {
        self.window.get(n) * self.prev.enhanced.get(l) * (
            self.prev.params.fundamental * n as f32 * l as f32 +
                self.prev.phase.get(l)
        ).cos()
    }

    fn get_pair(&self, l: usize, n: isize) -> f32 {
        match (self.voice.is_voiced(l), self.prev.voice.is_voiced(l)) {
            (false, false) => 0.0,
            (false, true) => self.sig_prev(l, n),
            (true, false) => self.sig_cur(l, n),
            (true, true) => if l >= 8 || self.freq_changed {
                self.sig_prev(l, n) + self.sig_cur(l, n)
            } else {
                self.amplitude(l, n) * self.theta(l, n).cos()
            },
        }
    }

    fn amplitude(&self, l: usize, n: isize) -> f32 {
        self.prev.enhanced.get(l) + n as f32 / SAMPLES as f32 *
            (self.enhanced.get(l) - self.prev.enhanced.get(l))
    }

    fn theta(&self, l: usize, n: isize) -> f32 {
        self.prev.phase.get(l) +
            (self.prev.params.fundamental * l as f32 + self.freq_change(l)) * n as f32 +
            (self.fundamental - self.prev.params.fundamental) *
                l as f32 * (n as f32).powi(2) / (2.0 * SAMPLES as f32)
    }

    fn phase_change(&self, l: usize) -> f32 {
        self.phase.get(l) - self.prev.phase.get(l) - (
            self.prev.params.fundamental + self.fundamental
        ) * l as f32 * SAMPLES as f32 / 2.0
    }

    fn freq_change(&self, l: usize) -> f32 {
        (SAMPLES as f32).recip() * (self.phase_change(l) - 2.0 * PI * (
            (self.phase_change(l) + PI) / (2.0 * PI)
        ).floor())
    }

    pub fn get(&self, n: usize) -> f32 {
        (1..self.end).map(|l| {
            2.0 * self.get_pair(l, n as isize)
        }).fold(0.0, |s, x| s + x)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use params::BaseParams;
    use prev::PrevFrame;
    use descramble::{descramble, Bootstrap};

    #[test]
    fn test_phase_base() {
        let chunks = [
            0b001000010010,
            0b110011001100,
            0b111000111000,
            0b111111111111,
            0b10101110101,
            0b00101111010,
            0b01110111011,
            0b00001000,
        ];

        let b = Bootstrap::new(&chunks);
        let p = BaseParams::new(b.unwrap_period());
        let prev = PrevFrame::default();

        assert!((p.fundamental - 0.17575344).abs() < 0.000001);
        assert!((prev.params.fundamental - 0.0937765407).abs() < 0.000001);

        let pb = PhaseBase::new(&p, &prev);

        assert!((pb.get(1) - -6.558151944).abs() < 0.0001);
        assert!((pb.get(2) - -13.11630389).abs() < 0.0001);
        assert!((pb.get(3) - -19.67445583).abs() < 0.0001);
        assert!((pb.get(4) - -26.23260778).abs() < 0.0001);
        assert!((pb.get(5) - -32.79075972).abs() < 0.0001);
        assert!((pb.get(6) - -39.34891166).abs() < 0.0001);
        assert!((pb.get(20) - -131.1630389).abs() < 0.0001);
        assert!((pb.get(56) - -367.2565089).abs() < 0.0001);
    }

    #[test]
    fn test_phase() {
        let chunks = [
            0b001000010010,
            0b110011001100,
            0b111000111000,
            0b111111111111,
            0b10101110101,
            0b00101111010,
            0b01110111011,
            0b00001000,
        ];

        let b = Bootstrap::new(&chunks);
        let p = BaseParams::new(b.unwrap_period());
        let prev = PrevFrame::default();
        let pb = PhaseBase::new(&p, &prev);
        let (_, voice, _) = descramble(&chunks, &p);
        let phase = Phase::new(&p, &voice, &pb);

        assert_eq!(voice.unvoiced_count, 2);

        assert!((phase.get(1) - -6.558151944).abs() < 0.0001);
        assert!((phase.get(2) - -13.11630389).abs() < 0.0001);
        assert!((phase.get(3) - -19.67445583).abs() < 0.0001);
        assert!((phase.get(4) - -26.23260778).abs() < 0.0001);
        assert!((phase.get(5) - -32.91586903).abs() < 0.0001);
        assert!((phase.get(6) - -39.37108022).abs() < 0.0001);
        assert!((phase.get(7) - -45.60512329).abs() < 0.0001);
    }
}