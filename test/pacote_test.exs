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

      assert {:ok, subject} = Pacote.csv_reader(path, ",")
      assert subject.destination_count == 5
      assert %{"phone" => _, "name" =>  _, "extra" => _} = subject.example_data
    end

    test "returns ok when valid data w/ blank column" do
      path = "test/files/valid_b.csv"

      assert {:ok, subject} = Pacote.csv_reader(path, ",")
      assert subject.destination_count == 5
    end

    test "returns error when only header" do
      path = "test/files/only_header.csv"

      assert Pacote.csv_reader(path, ",") == :no_rows_found
    end

    test "returns error when invalid csv" do
      path = "test/files/invalid_a.csv"
      assert Pacote.csv_reader(path, ",") == :row_format
    end

    test "returns error when empty csv" do
      path = "test/files/empty.csv"
      assert Pacote.csv_reader(path, ",") == :no_header
    end

    test "returns error when more column" do
      path = "test/files/invalid_b.csv"
      assert Pacote.csv_reader(path, ",") == :failed_to_read_first_row
    end
  end
end
