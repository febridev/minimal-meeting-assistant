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
            </div>
            <Button className="w-full">Save Changes</Button>
          </div>
        )}

        {activeSection === "ai" && (
          <div className="space-y-6">
            <div className="space-y-1">
              <Label htmlFor="api-key">OpenAI API Key</Label>
              <Input id="api-key" type="password" placeholder="sk-..." />
              <p className="text-xs text-muted-foreground">Your key is stored locally.</p>
            </div>
            <Button className="w-full">Save Changes</Button>
          </div>
        )}

        {activeSection === "about" && <AboutSection />}
      </main>
    </div>
  );
}