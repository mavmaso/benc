defmodule Pacote do
  use Rustler, otp_app: :benc, crate: "pacote"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
