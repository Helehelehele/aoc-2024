defmodule Aoc.Day03 do
  defp scan_input(input) do
    Regex.scan(~r/(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1,3})\)/, input)
  end

  defp process_match(["do()", _], {_enabled, sum, ignore_enable}) do
    {:enabled, sum, ignore_enable}
  end

  defp process_match(["don't()", _], {_enabled, sum, ignore_enable}) do
    {:disabled, sum, ignore_enable}
  end

  defp process_match([_, _, a, b], {enabled, sum, ignore_enable}) do
    if ignore_enable || enabled == :enabled do
      {
        :enabled,
        sum + String.to_integer(a) * String.to_integer(b),
        ignore_enable
      }
    else
      {:disabled, sum, ignore_enable}
    end
  end

  defp solve(path, ignore_enable \\ true) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.join()
    |> scan_input()
    |> Enum.reduce({:enabled, 0, ignore_enable}, &process_match/2)
    |> elem(1)
  end

  def solve_part1(path) do
    solve(path)
  end

  def solve_part2(path) do
    solve(path, false)
  end
end
