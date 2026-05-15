import { useState } from 'react';
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
} from "lucide-react";

export function RecordPopup({ onOpenSettings }: { onOpenSettings: () => void }) {
  const [isRecording, setIsRecording] = useState(false);

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
      <CardContent className="flex flex-col items-center justify-center pt-6 pb-8">
        <Button
          size="lg"
          className={`w-32 h-32 rounded-full ${isRecording ? 'bg-destructive' : 'bg-primary'}`}
          onClick={() => setIsRecording(!isRecording)}
        >
          {isRecording ? <Square className="h-12 w-12" /> : <Mic className="h-12 w-12" />}
        </Button>
        <p className="mt-4 text-sm font-medium">
          {isRecording ? "Recording..." : "Ready to record"}
        </p>
      </CardContent>
    </Card>
  );
}
