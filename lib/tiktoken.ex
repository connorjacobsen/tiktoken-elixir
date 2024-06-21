defmodule Tiktoken do
  @moduledoc """
  Documentation for `Tiktoken`.
  """

  @model_to_encoding %{
    "p50k_base" => Tiktoken.P50K,
    "p50k_edit" => Tiktoken.P50KEdit,
    "r50k_base" => Tiktoken.R50K,
    "cl100k_base" => Tiktoken.CL100K,
    "o200k_base" => Tiktoken.O200K
  }

  def encoding_for_model(model) do
    encoding_name = Tiktoken.Native.encoding_for_model(model)
    @model_to_encoding[encoding_name]
  end

  def encode_ordinary(model, text) do
    delegate_call(model, :encode_ordinary, [text])
  end

  def encode(model, text, allowed_special \\ []) do
    delegate_call(model, :encode, [text, allowed_special])
  end

  def encode_with_special_tokens(model, text) do
    delegate_call(model, :encode_with_special_tokens, [text])
  end

  def decode(model, ids) do
    delegate_call(model, :decode, [ids])
  end

  defp delegate_call(model, function, args) do
    if mod = encoding_for_model(model) do
      apply(mod, function, args)
    else
      {:error, {:unsupported_model, model}}
    end
  end

  # Those two can be removed when a release of tiktoken-rs > 0.5.8 is released
  def context_size_for_model("gpt-3.5-turbo-1106"), do: 16_385
  def context_size_for_model("gpt-4-0125-preview"), do: 128_000

  def context_size_for_model(model) do
    Tiktoken.Native.context_size_for_model(model)
  end
end
