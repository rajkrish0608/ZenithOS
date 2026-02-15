#![no_std]

extern crate alloc;
use alloc::string::String;

pub struct MicrophoneDriver;

impl MicrophoneDriver {
    /// Simulates listening to the microphone and performing Speech-to-Text (STT).
    pub fn listen(&self) -> String {
        // In a real OS, this would DMA audio samples and run a whisper.cpp inference.
        // For simulation, we return a hardcoded intent.
        String::from("Open Code Editor") 
    }

    /// Checks if the user is speaking.
    pub fn is_speaking(&self) -> bool {
        true // Mock: always detecting voice for this demo
    }
}
