import { css } from "../../../styled-system/css";

export const inputStyles = css({
  color: {
    base: "slate.800",
    _osDark: "slate.200",
  },
  bg: {
    base: "slate.200",
    _osDark: "slate.800",
  },
  borderWidth: "1px",
  borderColor: {
    base: "slate.300",
    _osDark: "slate.700",
    _focusVisible: "blue.500",
  },
  px: "2",
  py: "1",
  borderRadius: "md",
  outline: "none",
});

export const labelStyles = css({
  color: {
    base: "slate.600",
    _osDark: "slate.100",
  },
});
