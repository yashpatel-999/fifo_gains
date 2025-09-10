-- Add migration script here
-- create tables
CREATE TABLE IF NOT EXISTS transactions (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  product TEXT NOT NULL,
  txn_type TEXT NOT NULL,
  quantity INTEGER NOT NULL,
  price NUMERIC(20,4) NOT NULL
);

CREATE TABLE IF NOT EXISTS pnl_results (
  product TEXT PRIMARY KEY,
  pnl NUMERIC(30,8),
  computed_at TIMESTAMPTZ DEFAULT now()
);

-- insert sample rows (order & data from your assignment)
INSERT INTO transactions (date, product, txn_type, quantity, price) VALUES
('2021-01-01', 'Apple',  'Buy',  439, 28),
('2021-01-24', 'Apple',  'Sell', 255, 36),
('2021-02-05', 'Apple',  'Buy',  26,  42),
('2021-02-16', 'Orange', 'Buy',  356, 13),
('2021-03-06', 'Papaya', 'Buy',  75, 225),
('2021-03-08', 'Mango',  'Buy',  690, 47),
('2021-06-05', 'Apple',  'Sell', 200, 22),
('2021-08-27', 'Papaya', 'Sell', 45,  227),
('2021-09-03', 'Orange', 'Sell', 256, 9),
('2021-09-15', 'Mango',  'Sell', 540, 46.5),
('2021-10-04', 'Orange', 'Sell', 50,  18);