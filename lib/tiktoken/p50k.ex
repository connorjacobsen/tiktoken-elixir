defmodule Tiktoken.P50K do
  @behaviour Tiktoken.Encoding

  @impl Tiktoken.Encoding
  def encode_ordinary(text) do
    Tiktoken.Native.p50k_encode_ordinary(text)
  end

  @impl Tiktoken.Encoding
  def encode(text, allowed_special \\ []) do
    Tiktoken.Native.p50k_encode(text, allowed_special)
  end

  @impl Tiktoken.Encoding
  def encode_with_special_tokens(text) do
    Tiktoken.Native.p50k_encode_with_special_tokens(text)
  end

  @impl Tiktoken.Encoding
  def decode(ids) do
    Tiktoken.Native.p50k_decode(ids)
  end

  @impl Tiktoken.Encoding
  def count_tokens(text, allowed_special \\ []) do
    Tiktoken.Native.p50k_count_tokens(text, allowed_special)
  end
end
