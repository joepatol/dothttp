import * as vscode from "vscode";

const SEPARATOR_RE = /^###\s*(.*)$/;
const REQUEST_LINE_RE = /^(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE)\s+(\S+)/;

interface RequestEntry {
  line: number;
  identifier: string;
}

function findRequests(document: vscode.TextDocument): RequestEntry[] {
  const entries: RequestEntry[] = [];
  const lineCount = document.lineCount;
  let i = 0;

  let inNamedSection = false;

  while (i < lineCount) {
    const text = document.lineAt(i).text;
    const sepMatch = SEPARATOR_RE.exec(text);

    if (sepMatch !== null) {
      const name = sepMatch[1].trim();
      if (name !== "") {
        // Named request — identifier is the name, CodeLens on the separator line only
        entries.push({ line: i, identifier: name });
        inNamedSection = true;
      } else {
        // Separator without name — identifier will be METHOD URL of the next request line
        const reqLine = findNextRequestLine(document, i + 1);
        if (reqLine !== null) {
          entries.push({ line: i, identifier: reqLine.identifier });
        }
        inNamedSection = false;
      }
      i++;
      continue;
    }

    // Unnamed request not preceded by a separator — CodeLens goes on the method line itself
    const reqMatch = REQUEST_LINE_RE.exec(text);
    if (reqMatch !== null) {
      // Skip if already covered by a named or unnamed separator above
      if (!inNamedSection) {
        const identifier = `${reqMatch[1]} ${reqMatch[2]}`;
        const alreadyCovered = entries.some(
          (e) => e.identifier === identifier && e.line < i
        );
        if (!alreadyCovered) {
          entries.push({ line: i, identifier });
        }
      }
      inNamedSection = false;
    }

    i++;
  }

  return entries;
}

function findNextRequestLine(
  document: vscode.TextDocument,
  fromLine: number
): { identifier: string } | null {
  for (let i = fromLine; i < document.lineCount; i++) {
    const text = document.lineAt(i).text;
    const match = REQUEST_LINE_RE.exec(text);
    if (match) {
      return { identifier: `${match[1]} ${match[2]}` };
    }
    // Stop at the next separator
    if (SEPARATOR_RE.test(text)) {
      break;
    }
  }
  return null;
}

export class DothttpCodeLensProvider implements vscode.CodeLensProvider {
  private changeEmitter = new vscode.EventEmitter<void>();
  readonly onDidChangeCodeLenses = this.changeEmitter.event;

  constructor(private readonly context: vscode.ExtensionContext) {
    vscode.workspace.onDidChangeTextDocument(
      (_) => this.changeEmitter.fire(),
      null,
      context.subscriptions
    );
  }

  provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
    const requests = findRequests(document);
    return requests.map(({ line, identifier }) => {
      const range = new vscode.Range(line, 0, line, 0);
      return new vscode.CodeLens(range, {
        title: "▶ Send request",
        command: "dothttp.runRequest",
        arguments: [document.uri.fsPath, identifier],
      });
    });
  }
}
