import * as vscode from "vscode";

const MAX_BODY_BYTES = 1 * 1024 * 1024; // 1 MB

interface ParsedResponse {
  statusLine: string;
  headers: Array<[string, string]>;
  body: string;
  truncated: boolean;
}

function parseCliOutput(output: string): ParsedResponse {
  // The CLI prints: separator line, status line, headers, blank line, body
  // Example:
  //   ─── label ───
  //   HTTP 200
  //   content-type: application/json
  //
  //   {"key":"value"}
  const lines = output.split(/\r?\n/);

  let statusLine = "";
  const headers: Array<[string, string]> = [];
  let bodyLines: string[] = [];
  let state: "pre" | "status" | "headers" | "body" = "pre";

  for (const line of lines) {
    if (state === "pre") {
      // Skip the separator line (─── ... ───)
      if (line.trim().startsWith("───")) {
        state = "status";
      }
      continue;
    }
    if (state === "status") {
      statusLine = line.trim();
      state = "headers";
      continue;
    }
    if (state === "headers") {
      if (line.trim() === "") {
        state = "body";
        continue;
      }
      const colonIdx = line.indexOf(":");
      if (colonIdx > 0) {
        headers.push([line.slice(0, colonIdx).trim(), line.slice(colonIdx + 1).trim()]);
      }
      continue;
    }
    if (state === "body") {
      bodyLines.push(line);
    }
  }

  let body = bodyLines.join("\n").trimEnd();
  let truncated = false;

  const bodyBytes = Buffer.byteLength(body, "utf8");
  if (bodyBytes > MAX_BODY_BYTES) {
    body = Buffer.from(body, "utf8").subarray(0, MAX_BODY_BYTES).toString("utf8");
    truncated = true;
  }

  // Pretty-print JSON if possible
  try {
    const parsed = JSON.parse(body);
    body = JSON.stringify(parsed, null, 2);
  } catch {
    // not JSON, leave as-is
  }

  return { statusLine, headers, body, truncated };
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function statusColor(statusLine: string): string {
  const match = statusLine.match(/\d+/);
  if (!match) return "#cccccc";
  const code = parseInt(match[0], 10);
  if (code >= 200 && code < 300) return "#4ec9b0";
  if (code >= 300 && code < 400) return "#dcdcaa";
  if (code >= 400) return "#f48771";
  return "#cccccc";
}

function buildHtml(response: ParsedResponse): string {
  const color = statusColor(response.statusLine);
  const headerRows = response.headers
    .map(([k, v]) => `<tr><td class="hk">${escapeHtml(k)}</td><td>${escapeHtml(v)}</td></tr>`)
    .join("\n");

  const truncationNotice = response.truncated
    ? `<p class="truncation">Response body truncated at 1 MB.</p>`
    : "";

  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<style>
  body { font-family: var(--vscode-editor-font-family, monospace); font-size: 13px; padding: 12px; color: var(--vscode-editor-foreground); background: var(--vscode-editor-background); }
  .status { font-size: 1.2em; font-weight: bold; color: ${color}; margin-bottom: 12px; }
  table { border-collapse: collapse; margin-bottom: 12px; width: 100%; }
  td { padding: 2px 8px; vertical-align: top; }
  .hk { color: var(--vscode-symbolIcon-fieldForeground, #9cdcfe); font-weight: bold; white-space: nowrap; }
  pre { background: var(--vscode-textBlockQuote-background); padding: 10px; border-radius: 4px; overflow-x: auto; white-space: pre-wrap; word-break: break-word; }
  .truncation { color: var(--vscode-editorWarning-foreground, #cca700); font-style: italic; }
  h3 { margin: 8px 0 4px; color: var(--vscode-descriptionForeground); font-size: 0.85em; text-transform: uppercase; letter-spacing: 0.05em; }
</style>
</head>
<body>
<div class="status">${escapeHtml(response.statusLine)}</div>
${response.headers.length > 0 ? `<h3>Headers</h3><table>${headerRows}</table>` : ""}
${response.body ? `<h3>Body</h3>${truncationNotice}<pre>${escapeHtml(response.body)}</pre>` : ""}
</body>
</html>`;
}

export class ResponsePanel {
  private static current: ResponsePanel | undefined;
  private readonly panel: vscode.WebviewPanel;

  private constructor(panel: vscode.WebviewPanel) {
    this.panel = panel;
    panel.onDidDispose(() => {
      ResponsePanel.current = undefined;
    });
  }

  static show(output: string): void {
    if (ResponsePanel.current) {
      ResponsePanel.current.panel.reveal(vscode.ViewColumn.Beside);
    } else {
      const panel = vscode.window.createWebviewPanel(
        "dothttpResponse",
        "dothttp Response",
        vscode.ViewColumn.Beside,
        { enableScripts: false }
      );
      ResponsePanel.current = new ResponsePanel(panel);
    }

    const parsed = parseCliOutput(output);
    ResponsePanel.current.panel.webview.html = buildHtml(parsed);
  }
}
