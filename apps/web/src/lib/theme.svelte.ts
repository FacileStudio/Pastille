import { browser } from "$app/environment";

export const theme = $state<{ value: "light" | "dark" }>({
  value: browser && document.documentElement.classList.contains("light") ? "light" : "dark",
});

/**
 * Switches between the light and dark themes and remembers the choice.
 */
export function toggleTheme() {
  theme.value = theme.value === "dark" ? "light" : "dark";
  document.documentElement.classList.toggle("dark", theme.value === "dark");
  document.documentElement.classList.toggle("light", theme.value === "light");
  try {
    localStorage.setItem("pastille-theme", theme.value);
  } catch {}
}
