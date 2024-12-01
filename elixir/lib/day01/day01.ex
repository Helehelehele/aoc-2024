defmodule Aoc.Day01 do
  @spec build_lists(String.t()) :: {list(integer), list(integer)}
  defp build_lists(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.map(&String.split/1)
    |> Enum.map(fn list -> Enum.map(list, &String.to_integer/1) end)
    |> Enum.zip()
    |> Enum.map(&Tuple.to_list/1)
    |> List.to_tuple()
  end

  def solve_part1(path) do
    build_lists(path)
    |> Tuple.to_list()
    |> Enum.map(&Enum.sort/1)
    |> Enum.zip_with(fn [a, b] -> abs(a - b) end)
    |> Enum.sum()
  end

  def solve_part2(path) do
    build_lists(path)
    |> then(fn {first_list, second_list} ->
      first_list |> Enum.map(&find_similarity_score(&1, second_list))
    end)
    |> Enum.sum()
  end

  @spec find_similarity_score(integer, list(integer)) :: integer
  defp find_similarity_score(first_item, second_list) do
    second_list
    |> Enum.count(&(&1 == first_item))
    |> Kernel.*(first_item)
  end
end
