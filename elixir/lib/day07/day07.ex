defmodule Aoc.Day07 do
  @operators [:add, :multiply, :concat]

  def solve_part1(path) do
    parse_input(path)
    |> Enum.filter(&is_equation_valid?/1)
    |> Enum.map(fn {test_value, _} -> test_value end)
    |> Enum.sum()
  end

  def solve_part2(path) do
    parse_input(path)
    |> Enum.filter(&is_equation_valid?(&1, true))
    |> Enum.map(fn {test_value, _} -> test_value end)
    |> Enum.sum()
  end

  defp parse_input(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    [test_value, numbers] = String.split(line, ":")

    numbers =
      String.trim(numbers)
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    {String.to_integer(test_value), numbers}
  end

  defp is_equation_valid?({test_value, numbers}) do
    is_equation_valid?({test_value, numbers}, false)
  end

  defp is_equation_valid?({test_value, numbers}, allow_concat) do
    operator_count = length(numbers) - 1

    generate_operator_combinations(operator_count, allow_concat)
    |> Enum.any?(fn operators ->
      evaluate(numbers, operators) == test_value
    end)
  end

  defp generate_operator_combinations(count, allow_concat) do
    operators =
      if allow_concat,
        do: @operators,
        else: Enum.filter(@operators, &(&1 != :concat))

    for _ <- 1..count,
        reduce: [[]] do
      acc -> for op <- operators, comb <- acc, do: [op | comb]
    end
  end

  defp evaluate(numbers, operators) do
    [first | rest] = numbers

    Enum.zip(operators, rest)
    |> Enum.reduce(first, fn {operator, num}, acc ->
      case operator do
        :add ->
          acc + num

        :multiply ->
          acc * num

        :concat ->
          String.to_integer("#{acc}#{num}")
      end
    end)
  end
end
