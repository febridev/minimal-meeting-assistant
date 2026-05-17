import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function AboutSection() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h2 className="text-xl font-semibold">About</h2>
        <p className="text-sm text-muted-foreground">
          Information about Record Assistant.
        </p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Record Assistant</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4 text-sm">
          <div className="flex justify-between border-b pb-2">
            <span className="font-medium">Version</span>
            <span className="text-muted-foreground">0.0.1</span>
          </div>
          <div className="flex justify-between border-b pb-2">
            <span className="font-medium">Author</span>
            <span className="text-muted-foreground">
              Invisible Assistant Team
            </span>
          </div>
          <div className="flex justify-between border-b pb-2">
            <span className="font-medium">License</span>
            <span className="text-muted-foreground">MIT</span>
          </div>
          <div className="pt-2">
            <p className="leading-relaxed">
              A lightweight macOS assistant designed for seamless audio
              recording and AI-powered transcriptions/summaries of your
              meetings.
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
