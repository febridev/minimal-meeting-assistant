import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";
import { Switch } from "@/components/ui/switch";

export function SettingsPage() {
  return (
    <div className="p-6 max-w-2xl mx-auto space-y-6 bg-background text-foreground">
      <h1 className="text-2xl font-bold">Settings</h1>
      <Tabs defaultValue="general">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="general">General</TabsTrigger>
          <TabsTrigger value="ai">AI Config</TabsTrigger>
        </TabsList>
        <TabsContent value="general" className="space-y-4 pt-4">
          <div className="flex items-center justify-between">
            <Label htmlFor="launch">Launch at Login</Label>
            <Switch id="launch" />
          </div>
          <div className="space-y-1">
            <Label htmlFor="path">Save Path</Label>
            <Input id="path" placeholder="/Users/me/Documents/Recordings" />
          </div>
        </TabsContent>
        <TabsContent value="ai" className="space-y-4 pt-4">
          <div className="space-y-1">
            <Label htmlFor="api-key">OpenAI API Key</Label>
            <Input id="api-key" type="password" placeholder="sk-..." />
            <p className="text-xs text-muted-foreground">Your key is stored locally.</p>
          </div>
          <Button className="w-full">Save Changes</Button>
        </TabsContent>
      </Tabs>
    </div>
  );
}
