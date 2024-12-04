defmodule Aoc.Day04 do
  @type grid :: list(list(String.t()))
  @type position :: {integer, integer}
  @type direction :: {integer, integer}

  @valid_directions [
    {0, 1},
    {1, 0},
    {1, 1},
    {1, -1}
  ]

  def solve_part1(path) do
    solve(path, "XMAS")
  end

  def solve_part2(path) do
    solve(path, "MAS", true)
  end

  @spec count_word(grid(), String.t(), boolean()) :: integer
  defp count_word(grid, target_word, x_shape_only) do
    Enum.reduce(0..(length(grid) - 1), 0, fn row, acc ->
      acc +
        Enum.reduce(0..(length(Enum.at(grid, row)) - 1), 0, fn col, acc_inner ->
          acc_inner + count_matches(grid, target_word, {row, col}, x_shape_only)
        end)
    end)
  end

  @spec count_matches(grid(), String.t(), position(), boolean()) :: integer
  defp count_matches(grid, word, {row, col}, false) do
    Enum.count(@valid_directions, fn {dx, dy} ->
      check_word?(grid, word, {row, col}, {dx, dy}) ||
        check_word?(grid, String.reverse(word), {row, col}, {dx, dy})
    end)
  end

  defp count_matches(grid, word, {row, col}, true) do
    (check_x?(grid, word, row, col) && 1) || 0
  end

  defp check_word?(grid, word, {row, col}, {dx, dy}) do
    String.graphemes(word)
    |> Enum.with_index()
    |> Enum.all?(fn {char, i} ->
      is_in_bounds?(grid, row + i * dx, col + i * dy) &&
        get_char(grid, row + i * dx, col + i * dy) == char
    end)
  end

  defp check_x?(grid, word, row, col) do
    is_in_bounds?(
      grid,
      row + String.length(word) - 1,
      col + String.length(word) - 1
    ) &&
      (check_word?(grid, word, {row, col}, {1, 1}) ||
         check_word?(grid, String.reverse(word), {row, col}, {1, 1})) &&
      (check_word?(grid, word, {row, col + String.length(word) - 1}, {1, -1}) ||
         check_word?(grid, String.reverse(word), {row, col + String.length(word) - 1}, {1, -1}))
  end

  defp get_char(grid, row, col) do
    Enum.at(Enum.at(grid, row), col)
  end

  defp is_in_bounds?(grid, row, col) do
    row >= 0 && row < length(grid) && col >= 0 && col < length(Enum.at(grid, row))
  end

  defp solve(path, word, force_x \\ false) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&String.graphemes/1)
    |> count_word(word, force_x)
  end
end
