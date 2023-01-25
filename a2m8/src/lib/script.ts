export type Script = {
  id: string;
  name: string;
  description: string;
  startup: boolean;
  favorite: boolean;
  content: string;
  error?: string;
  status: scriptStatus;
  draft?: boolean;
};
export type scriptStatus = typeof scriptStatus[keyof typeof scriptStatus];
export const scriptStatus = {
  running: 1,
  stopped: 2,
  ended: 3,
  error: 4,
} as const;

export function statusToText(status: scriptStatus) {
  switch (status) {
    case scriptStatus.running:
      return "Running";
    case scriptStatus.stopped:
      return "Stopped";
    case scriptStatus.ended:
      return "Ended";
    case scriptStatus.error:
      return "Error";
  }
}
