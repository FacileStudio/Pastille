declare var Bun: {
  argv: string[];
  file(path: string): {
    json(): Promise<unknown>;
    text(): Promise<string>;
    arrayBuffer(): Promise<ArrayBuffer>;
    size: number;
  };
  write(path: string, data: string | Uint8Array): Promise<number>;
  Glob: new (pattern: string) => { scanSync(options: { cwd: string }): Iterable<string> };
  spawnSync(cmd: string[], options: { cwd: string }): { exitCode: number; stdout: Uint8Array };
};

interface ImportMeta {
  dir: string;
}
