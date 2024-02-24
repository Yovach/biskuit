import { css } from "../../../styled-system/css";

export const buttonStyles = css({
  transition: "all",
  transitionDuration: "250ms",
  bg: {
    base: "blue.500",
    _hover: "blue.600",
  },
  alignSelf: "flex-end",
  px: "3",
  py: "1.5",
  color: "blue.100",
  borderRadius: "lg",
  fontWeight: "semibold",
});
