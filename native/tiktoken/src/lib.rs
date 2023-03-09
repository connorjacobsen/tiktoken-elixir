use std::collections::HashSet;
use std::vec::Vec;

#[rustler::nif]
fn encoding_for_model(model: &str) -> Option<&str> {
    tiktoken_rs::encoding_for_model(model)
}

// p50k

#[rustler::nif]
fn p50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn p50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn p50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn p50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// p50k edit

#[rustler::nif]
fn p50k_edit_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn p50k_edit_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn p50k_edit_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn p50k_edit_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// r50k

#[rustler::nif]
fn r50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn r50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn r50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn r50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// cl100k

#[rustler::nif]
fn cl100k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn cl100k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn cl100k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn cl100k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

rustler::init!(
    "Elixir.Tiktoken.Native",
    [
        encoding_for_model,
        p50k_encode_ordinary,
        p50k_encode,
        p50k_encode_with_special_tokens,
        p50k_decode,
        p50k_edit_encode_ordinary,
        p50k_edit_encode,
        p50k_edit_encode_with_special_tokens,
        p50k_edit_decode,
        r50k_encode_ordinary,
        r50k_encode,
        r50k_encode_with_special_tokens,
        r50k_decode,
        cl100k_encode_ordinary,
        cl100k_encode,
        cl100k_encode_with_special_tokens,
        cl100k_decode
    ]
);
