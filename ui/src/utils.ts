export function dateString(timestamp: number): string {
  return `${new Date(timestamp * 1000).toLocaleTimeString().substring(0, 5)}h,
                  ${new Date(timestamp * 1000).toDateString()}`;
}
