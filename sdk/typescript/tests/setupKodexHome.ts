import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalKodexHome = process.env.KODEX_HOME;
let currentKodexHome: string | undefined;

beforeEach(async () => {
  currentKodexHome = await fs.mkdtemp(path.join(os.tmpdir(), "kodex-sdk-test-"));
  process.env.KODEX_HOME = currentKodexHome;
});

afterEach(async () => {
  const kodexHomeToDelete = currentKodexHome;
  currentKodexHome = undefined;

  if (originalKodexHome === undefined) {
    delete process.env.KODEX_HOME;
  } else {
    process.env.KODEX_HOME = originalKodexHome;
  }

  if (kodexHomeToDelete) {
    await fs.rm(kodexHomeToDelete, { recursive: true, force: true });
  }
});
