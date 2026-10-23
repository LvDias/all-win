// Stops the managed stack (volumes included) and removes the run directory, which may hold canary tokens.
import { rmSync } from 'node:fs';
import { config } from './config.ts';
import { compose, loadState } from './stack.ts';

export default async function globalTeardown(): Promise<void> {
  if (config.keepStack) {
    console.log(`E2E_KEEP_STACK=1: stack "${config.project}" and run directory ${config.runDir} kept`);
    return;
  }
  if (config.stack === 'managed') {
    try {
      const s = loadState();
      compose(s, ['down', '-v', '--remove-orphans'], { timeoutMs: 300_000, allowFail: true });
    } catch (e) {
      console.error(`teardown: ${(e as Error).message}`);
    }
  }
  rmSync(config.runDir, { recursive: true, force: true });
}
