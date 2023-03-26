defmodule NativeCsvTest do
  use ExUnit.Case, async: true

  describe "parse/2" do
    test "returns ok when valid data" do
      path = "test/files/valid_a.csv"

      assert list = NativeCsv.parse(path, ",")
      assert length(list) == 5
    end
  end
end
