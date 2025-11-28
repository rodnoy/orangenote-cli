//! FFI bindings for whisper.cpp
//!
//! This module provides low-level FFI bindings to the whisper.cpp C library.
//! These bindings are used internally by the whisper-rs wrapper crate.
//!
//! For high-level usage, see the `context` module which provides a safe Rust API.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_float, c_int, c_void};

/// Opaque pointer to a whisper context
pub type WhisperContext = c_void;

/// Opaque pointer to whisper state (for multithreading)
pub type WhisperState = c_void;

/// Whisper token type
pub type WhisperToken = i32;

/// Whisper sampling parameters
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WhisperFullParams {
    /// Strategy for sampling (0 = greedy, 1 = beam search)
    pub strategy: c_int,

    /// Number of threads to use for decoding
    pub n_threads: c_int,

    /// Maximum number of text tokens to generate per audio chunk
    pub n_max_text_ctx: c_int,

    /// Offset in seconds from the start of the audio
    pub offset_ms: c_int,

    /// Duration in milliseconds of audio to process (0 = to end)
    pub duration_ms: c_int,

    /// Translate to English
    pub translate: c_int,

    /// Don't use past transcripts as a guide
    pub no_context: c_int,

    /// Use past transcripts as a guide
    pub single_segment: c_int,

    /// Print special tokens
    pub print_special: c_int,

    /// Print progress
    pub print_progress: c_int,

    /// Print results from within `whisper_full`
    pub print_realtime: c_int,

    /// Print timestamps for each token
    pub print_timestamps: c_int,

    /// Token-level timestamps
    pub token_timestamps: c_int,

    /// Threshold for token-level timestamps (0.0 - 1.0, lower = get more timestamps)
    pub thold_pt: c_float,

    /// Threshold for segment-level timestamps (0.0 - 1.0, lower = get more timestamps)
    pub thold_ptsum: c_float,

    /// Maximum number of samples in a chunk (0 = disabled)
    pub max_len: c_int,

    /// Split on spaces
    pub split_on_word: c_int,

    /// Max tokens per segment from the last split point
    pub max_tokens: c_int,

    /// Language to use (ISO-639-1 code or NULL for auto-detect)
    pub language: *const c_char,

    /// Initial prompt
    pub initial_prompt: *const c_char,

    /// Callback function for progress
    pub progress_callback: Option<extern "C" fn(c_int, *mut c_void)>,

    /// Progress callback context
    pub progress_callback_user_data: *mut c_void,
}

impl Default for WhisperFullParams {
    fn default() -> Self {
        Self {
            strategy: 0,
            n_threads: 4,
            n_max_text_ctx: 16384,
            offset_ms: 0,
            duration_ms: 0,
            translate: 0,
            no_context: 0,
            single_segment: 0,
            print_special: 0,
            print_progress: 1,
            print_realtime: 0,
            print_timestamps: 0,
            token_timestamps: 0,
            thold_pt: 0.01,
            thold_ptsum: 0.01,
            max_len: 0,
            split_on_word: 0,
            max_tokens: 0,
            language: std::ptr::null(),
            initial_prompt: std::ptr::null(),
            progress_callback: None,
            progress_callback_user_data: std::ptr::null_mut(),
        }
    }
}

/// Result from segment transcription
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WhisperSegment {
    /// Segment index
    pub id: i32,

    /// Start time in 100-millisecond units
    pub t0: i64,

    /// End time in 100-millisecond units
    pub t1: i64,

    /// Text content
    pub text: *const c_char,

    /// Number of tokens in this segment
    pub n_tokens: i32,

    /// Token data pointer (for advanced usage)
    pub tokens: *const c_void,

    /// Probability of this being spoken
    pub p: c_float,
}

/// Token data
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WhisperTokenData {
    /// Token ID
    pub id: WhisperToken,

    /// Token ID used for next token search
    pub tid: WhisperToken,

    /// Probability
    pub p: c_float,

    /// Cumulative sum of probabilities
    pub psum: c_float,

    /// Probability of being voiced vs unvoiced
    pub pt: c_float,

    /// Cumulative sum of pt
    pub ptsum: c_float,

    /// Timestamp in 100-millisecond units
    pub t0: i64,

    /// Timestamp in 100-millisecond units
    pub t1: i64,

    /// Duration in 100-millisecond units
    pub tid_vprob: c_float,
}

// Link against whisper library when feature is enabled
// Build script (build.rs) sets up the library path and linking
#[cfg(feature = "whisper")]
#[link(name = "whisper")]
extern "C" {
    /// Initialize whisper context from file
    pub fn whisper_init_from_file(path: *const c_char) -> *mut WhisperContext;

    /// Initialize whisper context from buffer
    pub fn whisper_init_from_buffer(
        buffer: *const c_void,
        buffer_size: usize,
    ) -> *mut WhisperContext;

    /// Free whisper context
    pub fn whisper_free(ctx: *mut WhisperContext);

    /// Get default parameters
    pub fn whisper_full_default_params(strategy: c_int) -> WhisperFullParams;

    /// Run the full transcription pipeline
    pub fn whisper_full(
        ctx: *mut WhisperContext,
        params: WhisperFullParams,
        samples: *const c_float,
        n_samples: c_int,
    ) -> c_int;

    /// Run the full transcription pipeline with state
    pub fn whisper_full_with_state(
        ctx: *mut WhisperContext,
        state: *mut WhisperState,
        params: WhisperFullParams,
        samples: *const c_float,
        n_samples: c_int,
    ) -> c_int;

    /// Get number of segments
    pub fn whisper_full_n_segments(ctx: *mut WhisperContext) -> c_int;

    /// Get segment by index
    pub fn whisper_full_get_segment(ctx: *mut WhisperContext, i: c_int) -> WhisperSegment;

    /// Get segment text
    pub fn whisper_full_get_segment_text(ctx: *mut WhisperContext, i: c_int) -> *const c_char;

    /// Get segment start time in milliseconds
    pub fn whisper_full_get_segment_t0(ctx: *mut WhisperContext, i: c_int) -> i64;

    /// Get segment end time in milliseconds
    pub fn whisper_full_get_segment_t1(ctx: *mut WhisperContext, i: c_int) -> i64;

    /// Get segment probability
    pub fn whisper_full_get_segment_p(ctx: *mut WhisperContext, i: c_int) -> c_float;

    /// Get number of tokens in segment
    pub fn whisper_full_n_tokens(ctx: *mut WhisperContext, i: c_int) -> c_int;

    /// Get token data from segment
    pub fn whisper_full_get_token_data(
        ctx: *mut WhisperContext,
        i_segment: c_int,
        i_token: c_int,
    ) -> WhisperTokenData;

    /// Get token ID
    pub fn whisper_full_get_token_id(
        ctx: *mut WhisperContext,
        i_segment: c_int,
        i_token: c_int,
    ) -> WhisperToken;

    /// Get token text
    pub fn whisper_full_get_token_text(
        ctx: *mut WhisperContext,
        i_segment: c_int,
        i_token: c_int,
    ) -> *const c_char;

    /// Get token probability
    pub fn whisper_full_get_token_p(
        ctx: *mut WhisperContext,
        i_segment: c_int,
        i_token: c_int,
    ) -> c_float;

    /// Convert timestamp in 100-millisecond units to milliseconds
    pub fn whisper_full_get_segment_t0_ms(ctx: *mut WhisperContext, i: c_int) -> i64;

    /// Convert timestamp in 100-millisecond units to milliseconds
    pub fn whisper_full_get_segment_t1_ms(ctx: *mut WhisperContext, i: c_int) -> i64;

    /// Get language ID detected
    pub fn whisper_full_lang_id(ctx: *mut WhisperContext) -> c_int;

    /// Get language name by ID
    pub fn whisper_lang_str(id: c_int) -> *const c_char;

    /// Get language ID by name
    pub fn whisper_lang_id(lang: *const c_char) -> c_int;

    /// Create new state (for multithreading)
    pub fn whisper_state_new(ctx: *const WhisperContext) -> *mut WhisperState;

    /// Free state
    pub fn whisper_state_free(state: *mut WhisperState);

    /// Print system information
    pub fn whisper_print_system_info();
}
