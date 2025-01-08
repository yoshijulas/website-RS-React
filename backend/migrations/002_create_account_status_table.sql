CREATE TABLE IF NOT EXISTS public.account_status (
    id SERIAL PRIMARY KEY,
    status_name VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO public.account_status (status_name) VALUES ('active'), ('inactive'), ('banned');
