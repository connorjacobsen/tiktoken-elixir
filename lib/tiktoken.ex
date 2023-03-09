defmodule Tiktoken do
  @moduledoc """
  Documentation for `Tiktoken`.
  """

  @model_to_encoding %{
    "p50k_base" => Tiktoken.P50K,
    "p50k_edit" => Tiktoken.P50KEdit,
    "r50k_base" => Tiktoken.R50K,
    "cl100k_base" => Tiktoken.CL100K
  }

  def encoding_for_model(model) do
    encoding_name = Tiktoken.Native.encoding_for_model(model)
    @model_to_encoding[encoding_name]
  end
end
