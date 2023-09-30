import { test, expect } from "@playwright/test";

test("login", async ({ page }) => {
  await page.goto("https://hotel-seven-rho.vercel.app");
  await page.getByRole("link", { name: "Login" }).click();
  await page.getByLabel("Username").focus();
  await page.getByLabel("Username").fill("alice");
  await page.getByLabel("Username").press("Tab");
  await page.getByLabel("Password").fill("secret");
  await page.getByRole("button", { name: "Log in" }).click();
  await expect(page.getByText(/User: alice/)).toBeVisible();
});
