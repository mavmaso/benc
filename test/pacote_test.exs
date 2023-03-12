defmodule PacoteTest do
  use ExUnit.Case, async: true

  describe "add/2" do
    test "returns ok when valid data" do
      assert Pacote.add(1, 2) == 3
    end
  end

  describe "csv_reader/2" do
    test "returns ok when valid data" do
      path = "test/files/valid_a.csv"
      assert {:ok, %{destination_count: 5}} = Pacote.csv_reader(path, ",")
    end
  end
end
