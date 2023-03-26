defmodule NativeCsv do
  NimbleCSV.define(SemicolonParser, separator: ";", escape: "\"")
  NimbleCSV.define(TabParser, separator: "\t", escape: "\"")
  NimbleCSV.define(PipeParser, separator: "|", escape: "\"")
  NimbleCSV.define(CommaParser, separator: ",", escape: "\"")

  defp valid_parse(separator) do
    case separator do
      ";" -> SemicolonParser
      "\t" -> TabParser
      "|" -> PipeParser
      _ -> CommaParser
    end
  end

  def parse(path, separator) do
    path
    |> File.stream!()
    |> valid_parse(separator).parse_stream()
    |> Enum.to_list()
  end
end
