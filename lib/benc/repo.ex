defmodule Benc.Repo do
  use Ecto.Repo,
    otp_app: :benc,
    adapter: Ecto.Adapters.Postgres
end
