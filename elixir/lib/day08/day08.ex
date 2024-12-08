defmodule Aoc.Day08 do
  @type grid :: list(list(char()))
  @type position :: {integer, integer}
  @type antenna_positions :: %{char() => list(position())}

  def solve_part1(path) do
    parse_input(path)
    |> find_all_antinodes()
    |> Enum.map(&elem(&1, 1))
    |> Enum.uniq()
    |> Enum.count()
  end

  def solve_part2(path) do
    parse_input(path)
    |> find_all_antinodes(false)
    |> Enum.map(&elem(&1, 1))
    |> Enum.uniq()
    |> Enum.count()
  end

  @spec comb(integer, list(position())) :: list(list(position()))
  defp comb(0, _), do: [[]]
  defp comb(_, []), do: []

  defp comb(m, [h | t]) do
    for(l <- comb(m - 1, t), do: [h | l]) ++ comb(m, t)
  end

  defp parse_input(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&String.graphemes/1)
  end

  defp find_all_antinodes(grid, limit_distance \\ true) do
    find_antennas(grid)
    |> Enum.filter(fn {_, positions} -> length(positions) > 1 end)
    |> Enum.flat_map(fn {antenna, positions} ->
      comb(2, positions)
      |> Enum.flat_map(fn [pos1, pos2] -> find_antinodes(grid, pos1, pos2, limit_distance) end)
      |> Enum.map(fn pos -> {antenna, pos} end)
    end)
  end

  @spec find_antennas(grid()) :: antenna_positions()
  defp find_antennas(grid) do
    grid
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {row, row_index}, acc ->
      row
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {cell, col_index}, acc_inner ->
        case cell do
          "." ->
            acc_inner

          antenna ->
            Map.update(
              acc_inner,
              antenna,
              [{row_index, col_index}],
              &(&1 ++ [{row_index, col_index}])
            )
        end
      end)
    end)
  end

  @spec find_delta(position(), position()) :: position()
  defp find_delta(position1, position2) do
    {row1, col1} = position1
    {row2, col2} = position2

    {row2 - row1, col2 - col1}
  end

  @spec is_in_bounds?(grid(), position()) :: boolean()
  defp is_in_bounds?(grid, {row, col}) do
    grid
    |> get_grid_size()
    |> then(fn {rows, cols} -> row >= 0 and row < rows and col >= 0 and col < cols end)
  end

  @spec find_antinodes(grid(), position(), position(), boolean()) :: list(position())
  defp find_antinodes(grid, {p1x, p1y}, {p2x, p2y}, limit_distance) do
    {dx, dy} = find_delta({p1x, p1y}, {p2x, p2y})

    if limit_distance do
      [{p1x - dx, p1y - dy}, {p2x + dx, p2y + dy}]
      |> Enum.filter(&is_in_bounds?(grid, &1))
    else
      grid
      |> get_grid_size()
      |> calculate_max_steps(dx, dy)
      |> then(fn max_steps ->
        -max_steps..max_steps
        |> Enum.flat_map(fn i ->
          [
            {p1x - i * dx, p1y - i * dy},
            {p2x + i * dx, p2y + i * dy}
          ]
        end)
        |> Enum.filter(&is_in_bounds?(grid, &1))
        |> Enum.uniq()
      end)
    end
  end

  @spec get_grid_size(grid()) :: {integer, integer}
  defp get_grid_size(grid) do
    {length(grid), length(hd(grid))}
  end

  @spec calculate_max_steps({integer, integer}, integer, integer) :: integer
  defp calculate_max_steps({rows, cols}, dx, dy) do
    max(abs(dx), abs(dy))
    |> then(&(max(rows, cols) / &1))
    |> ceil()
    |> max(1)
  end
end
