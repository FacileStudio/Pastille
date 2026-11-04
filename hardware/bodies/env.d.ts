declare var Bun: {
  argv: string[];
  file(path: string): {
    json(): Promise<unknown>;
    text(): Promise<string>;
    arrayBuffer(): Promise<ArrayBuffer>;
    size: number;
  };
  write(path: string, data: string | Uint8Array): Promise<number>;
};

interface ImportMeta {
  dir: string;
}
