defmodule PacoteTest do
  use ExUnit.Case, async: true

  describe "add/2" do
    test "returns ok when valid data" do
      assert Pacote.add(1, 2) == 3
    end
  end
end
