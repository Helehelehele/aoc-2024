defmodule Aoc.Utils.ReadInput do
  def read_input(file_path) do
    File.read!("../input/" <> file_path)
    |> String.split("\n", trim: true)
  end
end
