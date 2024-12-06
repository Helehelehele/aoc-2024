defmodule Aoc.Day06 do
  @directions %{
    "^" => {-1, 0},
    ">" => {0, 1},
    "v" => {1, 0},
    "<" => {0, -1}
  }

  def solve_part1(path) do
    build_grid(path)
    |> then(fn grid -> {grid, find_guard(grid)} end)
    |> then(fn {grid, {direction, pos}} ->
      simulate_movement(grid, direction, pos, MapSet.new([pos]))
    end)
    |> MapSet.size()
  end

  def solve_part2(path) do
    grid = build_grid(path)
    {initial_direction, initial_pos} = find_guard(grid)

    all_positions =
      for row <- 0..(length(grid) - 1),
          col <- 0..(length(Enum.at(grid, 0)) - 1),
          not is_obstacle?(grid, {row, col}),
          {row, col} != initial_pos,
          do: {row, col}

    all_positions
    |> Enum.count(fn pos ->
      creates_loop?(grid, initial_direction, initial_pos, pos)
    end)
  end

  defp build_grid(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&String.graphemes/1)
  end

  defp find_guard(grid) do
    grid
    |> Enum.with_index()
    |> Enum.find_value(fn {row, row_idx} ->
      row
      |> Enum.with_index()
      |> Enum.find_value(fn {cell, cell_idx} ->
        if cell in Map.keys(@directions) do
          {
            cell,
            {row_idx, cell_idx}
          }
        else
          nil
        end
      end)
    end)
  end

  defp get_next_direction("^"), do: ">"
  defp get_next_direction(">"), do: "v"
  defp get_next_direction("v"), do: "<"
  defp get_next_direction("<"), do: "^"

  defp get_next_position({row, col}, direction) do
    {drow, dcol} = @directions[direction]
    {row + drow, col + dcol}
  end

  defp is_valid_position?(grid, {row, col}) do
    rows = length(grid)
    cols = length(Enum.at(grid, 0))
    row >= 0 and row < rows and col >= 0 and col < cols
  end

  defp is_obstacle?(grid, {row, col}) do
    Enum.at(grid, row) |> Enum.at(col) == "#"
  end

  defp simulate_movement(grid, direction, pos, visited) do
    pos
    |> get_next_position(direction)
    |> then(fn next_pos ->
      cond do
        not is_valid_position?(grid, next_pos) ->
          visited

        is_obstacle?(grid, next_pos) ->
          direction
          |> get_next_direction()
          |> then(fn new_direction ->
            simulate_movement(grid, new_direction, pos, visited)
          end)

        true ->
          simulate_movement(grid, direction, next_pos, MapSet.put(visited, next_pos))
      end
    end)
  end

  defp creates_loop?(grid, direction, start_pos, obstacle_pos) do
    simulate_with_obstacle(grid, direction, start_pos, obstacle_pos, MapSet.new([]), MapSet.new())
  end

  defp simulate_with_obstacle(
         grid,
         direction,
         pos,
         obstacle_pos,
         visited_positions,
         visited_states
       ) do
    next_pos = get_next_position(pos, direction)
    state = {pos, direction}

    cond do
      MapSet.member?(visited_states, state) ->
        true

      not is_valid_position?(grid, next_pos) ->
        false

      next_pos == obstacle_pos or is_obstacle?(grid, next_pos) ->
        new_direction = get_next_direction(direction)

        simulate_with_obstacle(
          grid,
          new_direction,
          pos,
          obstacle_pos,
          visited_positions,
          MapSet.put(visited_states, state)
        )

      true ->
        simulate_with_obstacle(
          grid,
          direction,
          next_pos,
          obstacle_pos,
          MapSet.put(visited_positions, next_pos),
          MapSet.put(visited_states, state)
        )
    end
  end
end
