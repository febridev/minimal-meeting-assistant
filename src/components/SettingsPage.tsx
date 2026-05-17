import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Settings, Sparkles, Info } from "lucide-react";
import { AboutSection } from "./AboutSection";

export function SettingsPage() {
  const [activeSection, setActiveSection] = useState("general");
  const [bitDepth, setBitDepth] = useState(() => {
    return localStorage.getItem("audioBitDepth") || "32";
  });

  const handleSave = () => {
    localStorage.setItem("audioBitDepth", bitDepth);
    alert("Settings saved!");
  };

  return (
    <div className="flex h-full bg-background">
      {/* Vertical Sidebar */}
      <nav className="flex flex-col w-48 border-r bg-muted/20 px-2 py-4 gap-4 overflow-hidden">
        <header className="px-3 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
          Settings
        </header>

        <ul className="space-y-1 flex-col">
          <li>
            <Button
              variant={activeSection === "general" ? "secondary" : "ghost"}
              className="w-full justify-start gap-2 h-9 px-3"
              onClick={() => setActiveSection("general")}
            >
              <Settings className="h-4 w-4" />
              General
            </Button>
          </li>
          <li>
            <Button
              variant={activeSection === "ai" ? "secondary" : "ghost"}
              className="w-full justify-start gap-2 h-9 px-3"
              onClick={() => setActiveSection("ai")}
            >
              <Sparkles className="h-4 w-4" />
              AI Config
            </Button>
          </li>
          <li>
            <Button
              variant={activeSection === "about" ? "secondary" : "ghost"}
              className="w-full justify-start gap-2 h-9 px-3"
              onClick={() => setActiveSection("about")}
            >
              <Info className="h-4 w-4" />
              About
            </Button>
          </li>
        </ul>
      </nav>

      {/* Main Content */}
      <main className="flex-1 p-6 overflow-hidden">
        {activeSection === "general" && (
          <div className="space-y-6">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <Label htmlFor="launch">Launch at Login</Label>
                <Switch id="launch" />
              </div>
              <div className="space-y-1">
                <Label htmlFor="path">Save Path</Label>
                <Input id="path" placeholder="/Users/me/Documents/Recordings" />
              </div>
              <div className="space-y-2 pt-2">
                <Label>Recording Quality</Label>
                <div className="flex gap-4 items-center">
                  <div className="flex items-center gap-2">
                    <input 
                      type="radio" 
                      id="32bit" 
                      name="bitDepth" 
                      value="32" 
                      checked={bitDepth === "32"} 
                      onChange={(e) => setBitDepth(e.target.value)}
                      className="w-4 h-4 text-primary"
                    />
                    <Label htmlFor="32bit" className="font-normal">32-bit (High Fidelity)</Label>
                  </div>
                  <div className="flex items-center gap-2">
                    <input 
                      type="radio" 
                      id="16bit" 
                      name="bitDepth" 
                      value="16" 
                      checked={bitDepth === "16"} 
                      onChange={(e) => setBitDepth(e.target.value)}
                      className="w-4 h-4 text-primary"
                    />
                    <Label htmlFor="16bit" className="font-normal">16-bit (Standard)</Label>
                  </div>
                </div>
                <p className="text-xs text-muted-foreground">Default is 32-bit for maximum clarity.</p>
              </div>
            </div>
            <Button className="w-full" onClick={handleSave}>Save Changes</Button>
          </div>
        )}

        {activeSection === "ai" && (
          <div className="space-y-6">
            <div className="space-y-1">
              <Label htmlFor="api-key">URL AI API </Label>
              <Input id="api-key" type="text" placeholder="https://..." />
              <p className="text-xs text-muted-foreground">
                Put Your URL API Here.
              </p>
            </div>
            <div className="space-y-1">
              <Label htmlFor="api-key">OpenAI API Key</Label>
              <Input id="api-key" type="password" placeholder="sk-..." />
              <p className="text-xs text-muted-foreground">
                Your key is stored locally.
              </p>
            </div>

            <Button className="w-full">Save Changes</Button>
          </div>
        )}

        {activeSection === "about" && <AboutSection />}
      </main>
    </div>
  );
}
