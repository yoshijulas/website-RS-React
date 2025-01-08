CREATE TABLE IF NOT EXISTS public.activity_logs (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    user_action TEXT NOT NULL,
    log_timestamp TIMESTAMP DEFAULT NOW()
);
