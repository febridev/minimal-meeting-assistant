import { useState } from "react";
import { RecordPopup } from "./components/RecordPopup";
import { SettingsPage } from "./components/SettingsPage";

function App() {
  const [showSettings, setShowSettings] = useState(false);

  return (
    <main className="flex min-h-screen items-center justify-center p-4 bg-background">
      {showSettings ? (
        <div className="w-full">
          <button 
            onClick={() => setShowSettings(false)}
            className="mb-4 text-sm text-muted-foreground hover:text-foreground"
          >
            ← Back
          </button>
          <SettingsPage />
        </div>
      ) : (
        <RecordPopup onOpenSettings={() => setShowSettings(true)} />
      )}
    </main>
  );
}

export default App;
