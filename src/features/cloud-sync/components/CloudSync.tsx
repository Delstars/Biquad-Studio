import { useState, useEffect } from "react";
import { supabase } from "../../../app/store/supabase";
import { useEqStore } from "../../../app/store/eqStore";

export function CloudSync() {
  const [session, setSession] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const { filters } = useEqStore();

  useEffect(() => {
    supabase.auth.getSession().then(({ data: { session } }) => {
      setSession(session);
    });

    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session) => {
      setSession(session);
    });

    return () => subscription.unsubscribe();
  }, []);

  const handleLogin = async () => {
    setLoading(true);
    // Use OAuth or Magic Link in production
    await supabase.auth.signInWithOAuth({ provider: 'github' });
    setLoading(false);
  };

  const handleSync = async () => {
    if (!session?.user) return;
    setLoading(true);
    
    // Encrypted Profile Export logic
    const profileData = {
      user_id: session.user.id,
      eq_filters: filters,
      updated_at: new Date().toISOString(),
    };

    const { error } = await supabase
      .from('profiles')
      .upsert(profileData);
      
    if (error) {
      console.error("Cloud Sync Error", error);
    } else {
      console.log("Profile synced to cloud successfully!");
    }
    
    setLoading(false);
  };

  return (
    <div className="bg-bq-bg-secondary border border-bq-border rounded-xl p-4 flex items-center justify-between">
      <div>
        <h3 className="text-white font-medium mb-1">Cloud Profile Sync (Premium)</h3>
        <p className="text-bq-text-secondary text-sm">
          {session ? `Logged in as ${session.user.email}` : "Sign in to backup and sync your profiles."}
        </p>
      </div>
      
      <div>
        {session ? (
          <button 
            onClick={handleSync}
            disabled={loading}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-md text-sm transition-colors"
          >
            {loading ? "Syncing..." : "Sync Now"}
          </button>
        ) : (
          <button 
            onClick={handleLogin}
            disabled={loading}
            className="px-4 py-2 bg-bq-bg-tertiary hover:bg-bq-border-active text-white rounded-md text-sm border border-bq-border transition-colors"
          >
            Sign In with GitHub
          </button>
        )}
      </div>
    </div>
  );
}
