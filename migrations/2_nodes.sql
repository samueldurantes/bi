CREATE TABLE nodes
(
  public_key text primary key    not null,
  alias      text                not null,
  capacity   numeric             not null,
  first_seen timestamptz         not null,
  created_at timestamptz         not null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('nodes');
