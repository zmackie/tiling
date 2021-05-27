/************************************************************************
************************************************************************
    FAUST Architecture File
    Copyright (C) 2017-2020 GRAME, Centre National de Creation Musicale
    ---------------------------------------------------------------------

    This is sample code. This file is provided as an example of minimal
    FAUST architecture file. Redistribution and use in source and binary
    forms, with or without modification, in part or in full are permitted.
    In particular you can create a derived work of this FAUST architecture
    and distribute that work under terms of your choice.

    This sample code is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

//! PortAudio architecture file

use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub struct Soundfile<'a> {
    fBuffers: &'a&'a F32,
    fLength: &'a i32,
    fSR: &'a i32,
    fOffset: &'a i32,
    fChannels: i32
}

pub trait FaustDsp {
    type T;

    fn new() -> Self where Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32) where Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}


fn mydsp_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
pub struct mydsp {
	fSampleRate: i32,
	fConst2: F32,
	iRec1: [i32;2],
	fConst3: F32,
	fConst4: F32,
	fRec0: [F32;3],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst2: 0.0,
			iRec1: [0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fRec0: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("filename", "exfaust1.dsp");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/resonlp:author", "Julius O. Smith III");
		m.declare("filters.lib/resonlp:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/resonlp:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("name", "exfaust1");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.0");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iRec1[l0 as usize] = 0;
		}
		for l1 in 0..3 {
			self.fRec0[l1 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::tan((1570.79639 / F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)))));
		let mut fConst1: F32 = (1.0 / fConst0);
		self.fConst2 = (1.0 / (((fConst1 + 0.200000003) / fConst0) + 1.0));
		self.fConst3 = (((fConst1 + -0.200000003) / fConst0) + 1.0);
		self.fConst4 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(fConst0))));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("exfaust1");
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, _inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iRec1[0] = ((1103515245 * self.iRec1[1]) + 12345);
			self.fRec0[0] = ((4.65661287e-10 * (self.iRec1[0] as F32)) - (self.fConst2 * ((self.fConst3 * self.fRec0[2]) + (self.fConst4 * self.fRec0[1]))));
			*output0 = ((self.fConst2 * (self.fRec0[2] + (self.fRec0[0] + (2.0 * self.fRec0[1])))) as F32);
			self.iRec1[1] = self.iRec1[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
		}
	}

}

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

// fn amain() {
//     run().unwrap()
// }

// fn arun() -> Result<(), pa::Error> {

//     let pa = pa::PortAudio::new()?;

//     // Allocation DSP on the heap
//     let mut dsp = Box::new(mydsp::new());

//     println!("Faust Rust code running with Portaudio: sample-rate = {} buffer-size = {}", SAMPLE_RATE, FRAMES_PER_BUFFER);

//     //Create a input/output stream with the same number of input and output channels
//     const INTERLEAVED: bool = false;// We want NON interleaved streams
//     let input_device = pa.default_input_device()?;
//     let output_device = pa.default_output_device()?;
//     let input_latency = pa.device_info(input_device)?.default_low_input_latency;
//     let output_latency = pa.device_info(output_device)?.default_low_input_latency;

//     let in_params = pa::StreamParameters::new(input_device, CHANNELS, INTERLEAVED, input_latency);
//     let out_params = pa::StreamParameters::new(output_device, CHANNELS, INTERLEAVED, output_latency);
//     let settings = pa::DuplexStreamSettings::new(in_params, out_params, SAMPLE_RATE, FRAMES_PER_BUFFER);
//     //This would have been interleaved:
//     //let mut settings = try!(pa.default_duplex_stream_settings(CHANNELS, CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));

//     println!("get_num_inputs: {}", dsp.get_num_inputs());
//     println!("get_num_outputs: {}", dsp.get_num_outputs());

//     // Init DSP with a given SR
//     dsp.init(SAMPLE_RATE as i32);

//     //settings.flags = pa::stream_flags::CLIP_OFF;

//     // This routine will be called by the PortAudio engine when audio is needed. It may called at
//     // interrupt level on some machines so don't do anything that could mess up the system like
//     // dynamic resource allocation or IO.
//     let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, frames, time, .. } : pa::DuplexStreamCallbackArgs<f32, f32>| {
//         let out_buffr: &mut [*mut f32];
//         let in_buffr: & [*const f32];
//         //rust-portaudio does not support non-interleaved audio out of the box (but portaudio does)
//         unsafe {
//             let out_buffer: *mut *mut f32 = ::std::mem::transmute(out_buffer.get_unchecked_mut(0));
//             out_buffr = ::std::slice::from_raw_parts_mut(out_buffer, CHANNELS as usize);
//             let output0 = ::std::slice::from_raw_parts_mut(out_buffr[0], frames);
//             let output1 = ::std::slice::from_raw_parts_mut(out_buffr[1], frames);

//             let in_buffer: *const *const f32 = ::std::mem::transmute(in_buffer.get_unchecked(0));
//             in_buffr = ::std::slice::from_raw_parts(in_buffer, CHANNELS as usize);
//             let input0 = ::std::slice::from_raw_parts(in_buffr[0], frames);
//             let input1 = ::std::slice::from_raw_parts(in_buffr[1], frames);

//             let inputs = &[input0, input1];
//             let outputs = &mut [output0, output1];

//             dsp.compute(frames as i32, inputs, outputs);
//         }
//         pa::Continue
//     };

//     let mut stream = pa.open_non_blocking_stream(settings, callback)?;

//     stream.start()?;

//     // Wait for user input to quit
//     println!("Press enter/return to quit...");
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input).ok();

//     stream.stop()?;
//     stream.close()?;

//     Ok(())
// }

fn main() {
    nannou::app(model).run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    freq: f64,
    volume: f32,
    dsp: mydsp,
}

fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window().view(view).build().unwrap();
    // Initialise the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialise the state that we want to live on the audio thread.
    let mut dsp = mydsp::new();
    dsp.init(SAMPLE_RATE as i32);
    let model = Audio {
        phase: 0.0,
        freq: 440.0,
        volume: 0.5,
        dsp: dsp,
    };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();
    Model { stream }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `freq`.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    // audio.volume = 0.5;

    // // play_sine(audio, buffer, freq, volume);
    // play_sine(audio, buffer, audio.freq, audio.volume);
    let inputs = &[];
    let length = 2;
    let mut outputs = buffer.frames_mut().collect::<Vec<&mut[f32]>>();
    
    audio.dsp.compute(length, inputs, &mut outputs)
}

fn play_sine(audio: &mut Audio, buffer: &mut Buffer, freq: f64, volume: f32) {
    let sample_rate = buffer.sample_rate() as f64;

    for frame in buffer.frames_mut() {
        // Create sine wave
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;

        // Progress phase according to speed of frequency
        audio.phase += freq / sample_rate;
        audio.phase %= sample_rate;

        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(PINK);
}
