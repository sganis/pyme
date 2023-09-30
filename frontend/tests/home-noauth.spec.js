// @ts-check
import { test, expect } from "@playwright/test";

let url = "https://hotel-seven-rho.vercel.app";

test("home links without auth", async ({ page }) => {
  await page.goto(url);
  await expect(page).toHaveTitle(/Hotel App/);
  await page.getByRole("link", { name: "Login" }).click();
  await expect(page.getByRole("heading", { name: "Login" })).toBeVisible();
  await page.getByRole("link", { name: "Register" }).click();
  await expect(page.getByRole("heading", { name: "Register" })).toBeVisible();
});

// test("login link", async ({ page }) => {
//   await page.goto(url);
//   await page.getByRole("link", { name: "Login" }).click();
//   await expect(page.getByRole("heading", { name: "Login" })).toBeVisible();
// });

// test("register link", async ({ page }) => {
//   await page.goto(url);
//   await page.getByRole("link", { name: "Register" }).click();
//   await expect(page.getByRole("heading", { name: "Register" })).toBeVisible();
// });

// test.describe("navigation", () => {
//   test.beforeEach(async ({ page }) => {
//     await page.goto(url);
//   });

//   test("main navigation", async ({ page }) => {
//     await expect(page).toHaveURL(url);
//   });
//   test("main navigation", async ({ page }) => {
//     await expect(page).toHaveURL(url);
//   });
//   test("register link", async ({ page }) => {
//     await page.getByRole("link", { name: "Register" }).click();
//     await expect(page.getByRole("heading", { name: "Register" })).toBeVisible();
//   });
// });
