declare var Bun: {
  argv: string[];
  file(path: string): { json(): Promise<unknown>; arrayBuffer(): Promise<ArrayBuffer> };
  write(path: string, data: string | Uint8Array): Promise<number>;
};

interface ImportMeta {
  dir: string;
}
