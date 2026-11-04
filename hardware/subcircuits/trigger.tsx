export const Trigger = () => (
  <>
    <testpoint
      name="TP_TOUCH"
      footprint="testpoint_round_2mm"
      pcbX={-12}
      pcbY={10}
    />
    <trace from=".TP_TOUCH > .pin1" to="net.TOUCH_WAKE" />
  </>
);
