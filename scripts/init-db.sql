-- Initialisation Space Conquest
-- gen_random_uuid() est built-in depuis PG13, pas besoin d'extension
-- pgcrypto reste utile pour argon2/bcrypt si besoin côté DB
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
