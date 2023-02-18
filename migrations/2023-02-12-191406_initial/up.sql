CREATE TABLE services ( 
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  secret TEXT NOT NULL,
  encrypted INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  last_used_at INTEGER
);
