defmodule Tiktoken.Encoding do
  @callback encode_ordinary(String.t()) :: {:ok, [integer()]} | {:error, String.t()}

  @callback encode(String.t(), [binary()]) :: {:ok, [integer()]} | {:error, String.t()}

  @callback encode_with_special_tokens(String.t()) :: {:ok, [integer()]} | {:error, String.t()}

  @callback decode([integer()]) :: {:ok, String.t()} | {:error, String.t()}
end
