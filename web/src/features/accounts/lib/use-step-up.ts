import { useStepUp } from "@/shared/session/step-up";
import type { StepUpAction } from "@/shared/api/types";
import type { StepUpProof } from "../api/types";

export type StepUpRunner = <T>(request: { action: StepUpAction; target?: string; description: string; send: (proof?: StepUpProof) => Promise<T> }) => Promise<T>;

/**
 * Runs the request without proof; when the Core answers `auth.step_up_required` the shared dialog asks for a code and
 * the request goes out exactly once more with `{ challenge_id, code }`. Rejects with `StepUpCancelled` on cancel.
 * Tests may inject another runner through the `stepUp` prop.
 */
export function useStepUpRunner(): StepUpRunner {
  const stepUp = useStepUp();
  return ({ action, target, description, send }) => stepUp.run({ action, targetId: target, description }, send);
}
