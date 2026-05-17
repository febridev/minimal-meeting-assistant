import { useState } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Mic,
  Square,
  Settings,
  ShieldCheck,
} from "lucide-react";

export function RecordPopup({ onOpenSettings }: { onOpenSettings: () => void }) {
  const [isRecording, setIsRecording] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [status, setStatus] = useState("Ready to record");

  const toggleRecording = async () => {
    try {
      if (!isRecording) {
        await invoke("start_recording");
        setIsRecording(true);
        setStatus("Recording...");
      } else {
        const bitDepth = parseInt(localStorage.getItem("audioBitDepth") || "32");
        const result = await invoke("stop_recording", { bitDepth });
        setIsRecording(false);
        setStatus("Summary saved!");
        console.log(result);
      }
    } catch (error) {
      console.error(error);
      setStatus("Error: " + error);
    }
  };

  const runCaptureTest = async () => {
    try {
      if (!isTesting) {
        setStatus("Select an app to record...");
        await invoke("start_recording");
        setIsTesting(true);
        setStatus("Capturing test audio...");
      } else {
        const bitDepth = parseInt(localStorage.getItem("audioBitDepth") || "32");
        const path = await invoke("debug_save_to_desktop", { bitDepth });
        setIsTesting(false);
        setStatus("Saved to Desktop!");
        alert(`Test file saved to: ${path}\nPlease verify the audio manually.`);
      }
    } catch (error) {
      console.error(error);
      setStatus("Test failed: " + error);
    }
  };

  return (
    <Card className="w-[300px] border-none shadow-none bg-background text-foreground">
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">Record Assistant</CardTitle>
        <Button
          variant="ghost"
          size="icon"
          onClick={onOpenSettings}
          className="flex h-4 w-4"
        >
          <Settings className="h-4 w-4" />
        </Button>
      </CardHeader>
      <CardContent className="flex flex-col items-center justify-center pt-6 pb-8 space-y-6">
        <div className="flex flex-col items-center">
          <Button
            size="lg"
            className={`w-32 h-32 rounded-full transition-all ${isRecording ? 'bg-destructive animate-pulse' : 'bg-primary'}`}
            onClick={toggleRecording}
            disabled={isTesting}
          >
            {isRecording ? <Square className="h-12 w-12" /> : <Mic className="h-12 w-12" />}
          </Button>
          <p className="mt-4 text-sm font-medium">
            {status}
          </p>
        </div>

        <Button
          variant="outline"
          size="sm"
          className="w-full gap-2"
          onClick={runCaptureTest}
          disabled={isRecording}
        >
          <ShieldCheck className="h-4 w-4" />
          {isTesting ? "Stop & Save to Desktop" : "Test Audio Capture"}
        </Button>
      </CardContent>
    </Card>
  );
}
