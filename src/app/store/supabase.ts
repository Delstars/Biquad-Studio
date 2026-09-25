import { createClient } from "@supabase/supabase-js";

// Note: In production, these should be securely injected via environment variables.
const SUPABASE_URL = "https://your-project-ref.supabase.co";
const SUPABASE_ANON_KEY = "your-anon-key";

export const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);
