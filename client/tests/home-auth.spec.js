import { test, expect } from "@playwright/test";

let url = "https://hotel-seven-rho.vercel.app";
test.use({
  storageState: "auth.json",
});

test.describe("home links after login", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(url);
  });

  test("user", async ({ page }) => {
    await page.getByRole("link", { name: "User" }).click();
    await expect(page.getByRole("heading", { name: "Users" })).toBeVisible();
  });

  test("role", async ({ page }) => {
    await page.getByRole("link", { name: "Role" }).click();
    await expect(page.getByRole("heading", { name: "Roles" })).toBeVisible();
  });

  test("booking", async ({ page }) => {
    await page.getByRole("link", { name: "Booking" }).click();
    await expect(page.getByRole("heading", { name: "Bookings" })).toBeVisible();
  });

  test("roomtype", async ({ page }) => {
    await page.getByRole("link", { name: "RoomType" }).click();
    await expect(
      page.getByRole("heading", { name: "Room Types" })
    ).toBeVisible();
  });

  test("room", async ({ page }) => {
    await page.getByRole("link", { name: "Room", exact: true }).click();
    await expect(page.getByRole("heading", { name: "Rooms" })).toBeVisible();
  });

  test("price", async ({ page }) => {
    await page.getByRole("link", { name: "Price" }).click();
    await expect(page.getByRole("heading", { name: "Prices" })).toBeVisible();
  });
});
