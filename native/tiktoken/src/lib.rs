use std::collections::HashSet;
use std::vec::Vec;

use tiktoken_rs::CoreBPE;
use tiktoken_rs::{cl100k_base, o200k_base, p50k_base, p50k_edit, r50k_base};

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

thread_local! {
    static R50K_BASE: CoreBPE = r50k_base().unwrap();
        static P50K_BASE: CoreBPE = p50k_base().unwrap();
        static P50K_EDIT: CoreBPE = p50k_edit().unwrap();
        static CL100K_BASE: CoreBPE = cl100k_base().unwrap();
        static O200K_BASE: CoreBPE = o200k_base().unwrap();
}

// p50k

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode_ordinary(text: &str) -> Result<Vec<u32>, String> {
    Ok(P50K_BASE.with(|bpe| bpe.encode_ordinary(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<u32>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    Ok(P50K_BASE.with(|bpe| bpe.encode(text, &set).0))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_encode_with_special_tokens(text: &str) -> Result<Vec<u32>, String> {
    Ok(P50K_BASE.with(|bpe| bpe.encode_with_special_tokens(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_decode(ids: Vec<u32>) -> Result<String, String> {
    P50K_BASE.with(|bpe| bpe.decode(ids).map_err(|e| e.to_string()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    Ok(P50K_BASE.with(|bpe| bpe.encode(text, &set).0.len()))
}

// p50k edit

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode_ordinary(text: &str) -> Result<Vec<u32>, String> {
    Ok(P50K_EDIT.with(|bpe| bpe.encode_ordinary(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<u32>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    Ok(P50K_EDIT.with(|bpe| bpe.encode(text, &set).0))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_encode_with_special_tokens(text: &str) -> Result<Vec<u32>, String> {
    Ok(P50K_EDIT.with(|bpe| bpe.encode_with_special_tokens(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_decode(ids: Vec<u32>) -> Result<String, String> {
    P50K_EDIT.with(|bpe| bpe.decode(ids).map_err(|e| e.to_string()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn p50k_edit_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    Ok(P50K_EDIT.with(|bpe| bpe.encode(text, &set).0.len()))
}

// r50k

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode_ordinary(text: &str) -> Result<Vec<u32>, String> {
    Ok(R50K_BASE.with(|bpe| bpe.encode_ordinary(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<u32>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    Ok(R50K_BASE.with(|bpe| bpe.encode(text, &set).0))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_encode_with_special_tokens(text: &str) -> Result<Vec<u32>, String> {
    Ok(R50K_BASE.with(|bpe| bpe.encode_with_special_tokens(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_decode(ids: Vec<u32>) -> Result<String, String> {
    R50K_BASE.with(|bpe| bpe.decode(ids).map_err(|e| e.to_string()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn r50k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    Ok(R50K_BASE.with(|bpe| bpe.encode(text, &set).0.len()))
}

// cl100k

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode_ordinary(text: &str) -> Result<Vec<u32>, String> {
    Ok(CL100K_BASE.with(|bpe| bpe.encode_ordinary(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<u32>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    Ok(CL100K_BASE.with(|bpe| bpe.encode(text, &set).0))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_encode_with_special_tokens(text: &str) -> Result<Vec<u32>, String> {
    Ok(CL100K_BASE.with(|bpe| bpe.encode_with_special_tokens(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_decode(ids: Vec<u32>) -> Result<String, String> {
    CL100K_BASE.with(|bpe| bpe.decode(ids).map_err(|e| e.to_string()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn cl100k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    Ok(CL100K_BASE.with(|bpe| bpe.encode(text, &set).0.len()))
}

// o200k

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode_ordinary(text: &str) -> Result<Vec<u32>, String> {
    Ok(O200K_BASE.with(|bpe| bpe.encode_ordinary(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<u32>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    Ok(O200K_BASE.with(|bpe| bpe.encode(text, &set).0))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_encode_with_special_tokens(text: &str) -> Result<Vec<u32>, String> {
    Ok(O200K_BASE.with(|bpe| bpe.encode_with_special_tokens(text)))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_decode(ids: Vec<u32>) -> Result<String, String> {
    O200K_BASE.with(|bpe| bpe.decode(ids).map_err(|e| e.to_string()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn o200k_count_tokens(text: &str, allowed_special: Vec<&str>) -> Result<usize, String> {
    let set: HashSet<&str> = allowed_special.into_iter().collect();
    Ok(O200K_BASE.with(|bpe| bpe.encode(text, &set).0.len()))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn context_size_for_model(model: &str) -> usize {
    tiktoken_rs::model::get_context_size(model)
}

rustler::init!("Elixir.Tiktoken.Native");
