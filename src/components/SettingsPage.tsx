import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Progress } from "@/components/ui/progress";
import { Settings, Sparkles, Info } from "lucide-react";
import { AboutSection } from "./AboutSection";

export function SettingsPage() {
  const [activeSection, setActiveSection] = useState("general");
  const [bitDepth, setBitDepth] = useState(() => {
    return localStorage.getItem("audioBitDepth") || "32";
  });
  const [whisperModelPath, setWhisperModelPath] = useState(
    () => localStorage.getItem("whisperModelPath") || ""
  );
  const [gemmaModelPath, setGemmaModelPath] = useState(
    () => localStorage.getItem("gemmaModelPath") || ""
  );

  const [whisperModel, setWhisperModel] = useState("tiny");
  const [gemmaModel, setGemmaModel] = useState("2b-it");
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [downloadStatus, setDownloadStatus] = useState("");

  useEffect(() => {
    let unlisten: () => void;
    async function setup() {
      unlisten = await listen<{ received: number; total: number }>("download-progress", (event) => {
        const { received, total } = event.payload;
        if (total) {
          setDownloadProgress(Math.round((received / total) * 100));
        }
      });
    }
    setup();
    return () => { if (unlisten) unlisten(); };
  }, []);

  const handleDownload = async (type: string, id: string) => {
    setIsDownloading(true);
    setDownloadProgress(0);
    setDownloadStatus(`Downloading ${type} model...`);
    
    try {
      const path = await invoke<string>("download_model", { modelType: type, modelId: id });
      if (type === "whisper") {
        setWhisperModelPath(path);
      } else {
        setGemmaModelPath(path);
      }
      alert(`Successfully downloaded ${type} model to ${path}`);
    } catch (e) {
      alert(`Failed to download: ${e}`);
    } finally {
      setIsDownloading(false);
      setDownloadStatus("");
    }
  };

  const handleSave = () => {
    localStorage.setItem("audioBitDepth", bitDepth);
    localStorage.setItem("whisperModelPath", whisperModelPath);
    localStorage.setItem("gemmaModelPath", gemmaModelPath);
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
            <Card className="p-4 space-y-4">
              <div className="space-y-2">
                <Label>Whisper Model</Label>
                <div className="flex items-center gap-2">
                  <select
                    value={whisperModel}
                    onChange={(e) => setWhisperModel(e.target.value)}
                    className="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                  >
                    <option value="tiny">Tiny (Fastest, Lower Accuracy)</option>
                    <option value="base">Base (Moderate)</option>
                    <option value="small">Small (Slower, Higher Accuracy)</option>
                  </select>
                  <Button onClick={() => handleDownload('whisper', whisperModel)}>Download</Button>
                </div>
                <Input
                  id="whisper-model-path"
                  type="text"
                  placeholder="Or enter path to Whisper model"
                  value={whisperModelPath}
                  onChange={(e) => setWhisperModelPath(e.target.value)}
                />
              </div>

              <div className="space-y-2">
                <Label>Gemma Model</Label>
                <div className="flex items-center gap-2">
                  <select
                    value={gemmaModel}
                    onChange={(e) => setGemmaModel(e.target.value)}
                    className="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                  >
                    <option value="2b-it">2B-IT (General Purpose)</option>
                  </select>
                  <Button onClick={() => handleDownload('gemma', gemmaModel)}>Download</Button>
                </div>
                <Input
                  id="gemma-model-path"
                  type="text"
                  placeholder="Or enter path to Gemma model"
                  value={gemmaModelPath}
                  onChange={(e) => setGemmaModelPath(e.target.value)}
                />
              </div>
              
              {isDownloading && (
                <div className="space-y-2">
                  <p className="text-sm text-muted-foreground">{downloadStatus}</p>
                  <Progress value={downloadProgress} />
                </div>
              )}
            </Card>

            <Button className="w-full" onClick={handleSave}>
              Save AI Configuration
            </Button>
          </div>
        )}

        {activeSection === "about" && <AboutSection />}
      </main>
    </div>
  );
}
