defmodule Tiktoken.CL100kTest do
  use ExUnit.Case
  alias Tiktoken.CL100K

  test "encode" do
    {:ok, tokens} = CL100K.encode("hello world")
    assert tokens == [15339, 1917]
  end

  test "encode_ordinary" do
    {:ok, tokens} = CL100K.encode_ordinary("hello world")
    assert tokens == [15339, 1917]
  end

  test "encode_with_special_tokens" do
    {:ok, tokens} = CL100K.encode_with_special_tokens("H0w @re You?")
    assert tokens == [39, 15, 86, 571, 265, 1472, 30]
  end

  test "decode" do
    {:ok, tokens} = CL100K.decode([39, 15, 86, 571, 265, 1472, 30])
    assert tokens == "H0w @re You?"
  end
end
