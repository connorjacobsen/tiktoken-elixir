mod thread_local;

use std::collections::HashSet;
use std::vec::Vec;
use thread_local::*;

#[rustler::nif(schedule = "DirtyCpu")]
fn encoding_for_model(model: &str) -> Option<&str> {
    match tiktoken_rs::tokenizer::get_tokenizer(model) {
        Some(tiktoken_rs::tokenizer::Tokenizer::O200kBase) => Some("o200k_base"),
        Some(tiktoken_rs::tokenizer::Tokenizer::Cl100kBase) => Some("cl100k_base"),
        Some(tiktoken_rs::tokenizer::Tokenizer::P50kBase) => Some("p50k_base"),
        Some(tiktoken_rs::tokenizer::Tokenizer::R50kBase) => Some("r50k_base"),
        Some(tiktoken_rs::tokenizer::Tokenizer::P50kEdit) => Some("p50k_edit"),
        _ => None,
    }
}

// p50k

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::p50k_base_thread_local();
    Ok(bpe.encode_ordinary(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = crate::p50k_base_thread_local();
    Ok(bpe.encode(text, set))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::p50k_base_thread_local();
    Ok(bpe.encode_with_special_tokens(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = crate::p50k_base_thread_local();
    bpe.decode(ids).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    let bpe = crate::p50k_base_thread_local();
    Ok(bpe.encode(text, set).len())
}

// p50k edit

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::p50k_edit_thread_local();
    Ok(bpe.encode_ordinary(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = crate::p50k_edit_thread_local();
    Ok(bpe.encode(text, set))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::p50k_edit_thread_local();
    Ok(bpe.encode_with_special_tokens(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = crate::p50k_edit_thread_local();
    bpe.decode(ids).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    let bpe = crate::p50k_edit_thread_local();
    Ok(bpe.encode(text, set).len())
}

// r50k

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::r50k_base_thread_local();
    Ok(bpe.encode_ordinary(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = crate::r50k_base_thread_local();
    Ok(bpe.encode(text, set))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::r50k_base_thread_local();
    Ok(bpe.encode_with_special_tokens(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = crate::r50k_base_thread_local();
    bpe.decode(ids).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    let bpe = crate::r50k_base_thread_local();
    Ok(bpe.encode(text, set).len())
}

// cl100k

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::cl100k_base_thread_local();
    Ok(bpe.encode_ordinary(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = crate::cl100k_base_thread_local();
    Ok(bpe.encode(text, set))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::cl100k_base_thread_local();
    Ok(bpe.encode_with_special_tokens(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = crate::cl100k_base_thread_local();
    bpe.decode(ids).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    let bpe = crate::cl100k_base_thread_local();
    Ok(bpe.encode(text, set).len())
}

// o200k

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::o200k_base_thread_local();
    Ok(bpe.encode_ordinary(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = crate::o200k_base_thread_local();
    Ok(bpe.encode(text, set))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = crate::o200k_base_thread_local();
    Ok(bpe.encode_with_special_tokens(text))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = crate::o200k_base_thread_local();
    bpe.decode(ids).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    let bpe = crate::o200k_base_thread_local();
    Ok(bpe.encode(text, set).len())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn context_size_for_model(model: &str) -> usize {
    tiktoken_rs::model::get_context_size(model)
}

rustler::init!(
    "Elixir.Tiktoken.Native",
    [
        encoding_for_model,
        p50k_encode_ordinary,
        p50k_encode,
        p50k_encode_with_special_tokens,
        p50k_decode,
        p50k_count_tokens,
        p50k_edit_encode_ordinary,
        p50k_edit_encode,
        p50k_edit_encode_with_special_tokens,
        p50k_edit_decode,
        p50k_edit_count_tokens,
        r50k_encode_ordinary,
        r50k_encode,
        r50k_encode_with_special_tokens,
        r50k_decode,
        r50k_count_tokens,
        cl100k_encode_ordinary,
        cl100k_encode,
        cl100k_encode_with_special_tokens,
        cl100k_decode,
        cl100k_count_tokens,
        o200k_encode_ordinary,
        o200k_encode,
        o200k_encode_with_special_tokens,
        o200k_decode,
        o200k_count_tokens,
        context_size_for_model
    ]
);
