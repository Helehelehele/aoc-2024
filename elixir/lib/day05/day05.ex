defmodule Aoc.Day05 do
  def solve_part1(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.split_while(&String.contains?(&1, "|"))
    |> then(fn {rules, pages} -> {parse_rules(rules), parse_pages(pages)} end)
    |> then(fn {rules, pages} ->
      pages
      |> Enum.filter(&is_correct?(rules, &1))
    end)
    |> Enum.map(&get_middle_page/1)
    |> Enum.sum()
  end

  def solve_part2(path) do
    Aoc.Utils.ReadInput.read_input(path)
    |> Enum.split_while(&String.contains?(&1, "|"))
    |> then(fn {rules, pages} -> {parse_rules(rules), parse_pages(pages)} end)
    |> then(fn {rules, pages} ->
      {rules, pages |> Enum.filter(&(!is_correct?(rules, &1)))}
    end)
    |> then(fn {rules, pages} -> pages |> Enum.map(&reorder({rules, &1})) end)
    |> Enum.map(&get_middle_page/1)
    |> Enum.sum()
  end

  defp reorder({rules, page}) do
    page
    |> Enum.sort(fn a, b ->
      cond do
        # If a must come before b according to rules
        Map.get(rules, a, []) |> Enum.member?(b) -> true
        # If b must come before a according to rules
        Map.get(rules, b, []) |> Enum.member?(a) -> false
        # If no direct rule exists, maintain original order
        true -> a <= b
      end
    end)
  end

  defp get_middle_page(pages) do
    Enum.at(pages, div(Enum.count(pages), 2))
  end

  defp is_correct?(rules, pages) do
    rules
    |> Enum.all?(fn {before, after_list} ->
      after_list
      |> Enum.all?(fn after_page ->
        check_order(pages, before, after_page)
      end)
    end)
  end

  defp check_order(pages, before, after_page) do
    case {Enum.member?(pages, before), Enum.member?(pages, after_page)} do
      {true, true} ->
        before_idx = Enum.find_index(pages, &(&1 == before))
        after_idx = Enum.find_index(pages, &(&1 == after_page))
        before_idx < after_idx

      _ ->
        true
    end
  end

  defp parse_pages(lines) do
    lines
    |> Enum.map(&parse_page/1)
  end

  defp parse_page(line) do
    line
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
  end

  defp parse_rules(lines) do
    lines
    |> Enum.map(&parse_rule/1)
    |> Enum.reduce(%{}, fn {k, v}, acc ->
      Map.update(acc, k, [v], &[v | &1])
    end)
  end

  defp parse_rule(line) do
    line
    |> String.split("|")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end
