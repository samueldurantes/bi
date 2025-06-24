CREATE TABLE nodes
(
  public_key text primary unique not null,
  alias                   unique not null,
  capacity   numeric(16,8)       not null,
  first_seen timestamptz         not null,
  created_at timestamptz         not null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('nodes');
