defmodule Aoc.Day02 do
  @spec comb(integer, list(integer)) :: list(list(integer))
  defp comb(0, _), do: [[]]
  defp comb(_, []), do: []

  defp comb(m, [h | t]) do
    for(l <- comb(m - 1, t), do: [h | l]) ++ comb(m, t)
  end

  @spec build_reports(String.t()) :: list(list(integer))
  defp build_reports(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report -> Enum.map(report, &String.to_integer/1) end)
  end

  @spec is_report_safe(list(integer)) :: boolean
  defp is_report_safe(report) do
    Enum.zip(report, Enum.drop(report, 1))
    |> Enum.all?(fn {a, b} -> (b - a) in [-3, -2, -1] end) ||
      Enum.zip(report, Enum.drop(report, 1))
      |> Enum.all?(fn {a, b} -> (b - a) in [1, 2, 3] end)
  end

  def solve_part1(path) do
    build_reports(path)
    |> Enum.filter(&is_report_safe/1)
    |> Enum.count()
  end

  defp is_report_safe_with_dampener(report) do
    is_report_safe(report) ||
      comb(length(report) - 1, report)
      |> Enum.any?(&is_report_safe/1)
  end

  def solve_part2(path) do
    build_reports(path)
    |> Enum.filter(&is_report_safe_with_dampener/1)
    |> Enum.count()
  end
end
