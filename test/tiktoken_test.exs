defmodule TiktokenTest do
  use ExUnit.Case
  doctest Tiktoken

  describe "encoding_for_model/1" do
    test "get the proper module for supported model" do
      [
        # chat
        {"gpt-3.5-turbo", Tiktoken.CL100K},
        # text
        {"text-davinci-003", Tiktoken.P50K},
        {"text-davinci-002", Tiktoken.P50K},
        {"text-davinci-001", Tiktoken.R50K},
        {"text-curie-001", Tiktoken.R50K},
        {"text-babbage-001", Tiktoken.R50K},
        {"text-ada-001", Tiktoken.R50K},
        {"davinci", Tiktoken.R50K},
        {"curie", Tiktoken.R50K},
        {"babbage", Tiktoken.R50K},
        {"ada", Tiktoken.R50K},
        # code
        {"code-davinci-002", Tiktoken.P50K},
        {"code-davinci-001", Tiktoken.P50K},
        {"code-cushman-002", Tiktoken.P50K},
        {"code-cushman-001", Tiktoken.P50K},
        {"davinci-codex", Tiktoken.P50K},
        {"cushman-codex", Tiktoken.P50K},
        # edit
        {"text-davinci-edit-001", Tiktoken.P50KEdit},
        {"code-davinci-edit-001", Tiktoken.P50KEdit},
        # embeddings
        {"text-embedding-ada-002", Tiktoken.CL100K},
        # old embeddings
        {"text-similarity-davinci-001", Tiktoken.R50K},
        {"text-similarity-curie-001", Tiktoken.R50K},
        {"text-similarity-babbage-001", Tiktoken.R50K},
        {"text-similarity-ada-001", Tiktoken.R50K},
        {"text-search-davinci-doc-001", Tiktoken.R50K},
        {"text-search-curie-doc-001", Tiktoken.R50K},
        {"text-search-babbage-doc-001", Tiktoken.R50K},
        {"text-search-ada-doc-001", Tiktoken.R50K},
        {"code-search-babbage-code-001", Tiktoken.R50K},
        {"code-search-ada-code-001", Tiktoken.R50K}
        # open source
        # {"gpt2", "gpt2"}
      ]
      |> Enum.each(fn {model, mod} ->
        assert Tiktoken.encoding_for_model(model) == mod
      end)
    end

    test "get nil for unsupported model" do
      assert is_nil(Tiktoken.encoding_for_model("gpt2"))
    end
  end

  describe "encode_ordinary/2" do
    test "with supported model" do
      assert {:ok, ids} =
               Tiktoken.encode_ordinary("gpt-3.5-turbo", "Tell me more about Elixir!")

      assert length(ids) == 7
    end

    test "with unsupported model" do
      assert {:error, {:unsupported_model, "gpt2"}} =
               Tiktoken.encode_ordinary("gpt2", "Tell me more about Elixir!")
    end
  end

  describe "encode/2" do
    test "with supported model" do
      assert {:ok, ids} =
               Tiktoken.encode("gpt-3.5-turbo", "Tell me more about Elixir!")

      assert length(ids) == 7
    end

    test "with unsupported model" do
      assert {:error, {:unsupported_model, "gpt2"}} =
               Tiktoken.encode("gpt2", "Tell me more about Elixir!")
    end
  end

  describe "encode_with_special_tokens/2" do
    test "with supported model" do
      assert {:ok, ids} =
               Tiktoken.encode_with_special_tokens("gpt-3.5-turbo", "Tell me more about Elixir!")

      assert length(ids) == 7
    end

    test "with unsupported model" do
      assert {:error, {:unsupported_model, "gpt2"}} =
               Tiktoken.encode_with_special_tokens("gpt2", "Tell me more about Elixir!")
    end
  end

  describe "decode/2" do
    test "with supported model" do
      text = "Tell me more about Elixir!"

      assert {:ok, ids} =
               Tiktoken.encode("gpt-3.5-turbo", text)

      assert {:ok, ^text} =
               Tiktoken.decode("gpt-3.5-turbo", ids)
    end

    test "with unsupported model" do
      assert {:error, {:unsupported_model, "gpt2"}} =
               Tiktoken.decode("gpt2", [1])
    end
  end
end
