export function dateString(timestamp: number): string {
  return `${new Date(timestamp * 1000).toLocaleTimeString()}h,
                  ${new Date(timestamp * 1000).toDateString()}`;
}
