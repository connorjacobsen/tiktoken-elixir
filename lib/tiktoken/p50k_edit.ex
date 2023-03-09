defmodule Tiktoken.P50KEdit do
  @behaviour Tiktoken.Encoding

  @impl Tiktoken.Encoding
  def encode_ordinary(text) do
    Tiktoken.Native.p50k_edit_encode_ordinary(text)
  end

  @impl Tiktoken.Encoding
  def encode(text, allowed_special \\ []) do
    Tiktoken.Native.p50k_edit_encode(text, allowed_special)
  end

  @impl Tiktoken.Encoding
  def encode_with_special_tokens(text) do
    Tiktoken.Native.p50k_edit_encode_with_special_tokens(text)
  end

  @impl Tiktoken.Encoding
  def decode(ids) do
    Tiktoken.Native.p50k_edit_decode(ids)
  end
end
