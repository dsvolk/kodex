import path from "node:path";

export function kodexPathOverride() {
  return (
    process.env.KODEX_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "kodex-rs", "target", "debug", "kodex")
  );
}
