CREATE TABLE nodes
(
  public_key text primary key    not null,
  is_enabled boolean             not null,
  alias      text         unique not null,
  capacity   numeric(16,8)       not null,
  first_seen timestamptz         not null,
  created_at timestamptz         not null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('nodes');
