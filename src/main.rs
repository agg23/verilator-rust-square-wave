extern crate verilated;
extern crate verilated_module;

use hound::{WavSpec, WavWriter};
use verilated_module::module;

#[module(top)]
pub struct Top {
    #[port(clock)]
    pub clk: bool,
    #[port(output)]
    pub audio: bool,
}

pub fn main() {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };

    let mut writer = WavWriter::create("output.wav", spec).expect("Couldn't create writer");

    let mut tb = Top::default();
    tb.eval();

    let mut sample_count = 0;
    writer.write_sample(0.0).expect("Could not write initial sample");

    for counter in 0..300_000_000 {
        tb.clock_toggle();
        tb.eval();
        tb.clock_toggle();
        tb.eval();

        // 100MHz / 44100Hz
        if counter % (100_000_000 / 44_100) == 0 {
            writer.write_sample(if tb.audio() == 1 {0.5} else {0.0}).expect("Could not write sample");
            sample_count += 1;
        }
    }

    writer.finalize().expect("Could not save wav");

    println!("Wrote {sample_count} samples");
}
