import { useState } from "react";
import { RecordPopup } from "./components/RecordPopup";
import { SettingsPage } from "./components/SettingsPage";
import { Button } from "@/components/ui/button";
import { ChevronLeft } from "lucide-react";

function App() {
  const [showSettings, setShowSettings] = useState(false);

  return (
    <main className="flex min-h-screen items-center justify-center p-4 bg-muted/10">
      {showSettings ? (
        <div className="w-full max-w-4xl h-[600px] bg-background rounded-xl border shadow-2xl overflow-hidden flex flex-col">
          <div className="p-2 border-b flex items-center bg-muted/30">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowSettings(false)}
              className="flex items-center gap-1 text-sm h-8"
            >
              <ChevronLeft className="h-4 w-4" />
              Back to Recording
            </Button>
          </div>
          <div className="flex-1 overflow-hidden">
            <SettingsPage />
          </div>
        </div>
      ) : (
        <RecordPopup onOpenSettings={() => setShowSettings(true)} />
      )}
    </main>
  );
}

export default App;
