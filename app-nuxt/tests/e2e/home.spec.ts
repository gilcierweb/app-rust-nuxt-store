import { expect, test } from '@nuxt/test-utils/playwright'

test.describe('Home Page', () => {
  test('should load the home page successfully', async ({ page, goto }) => {
    // This will start Nuxt and navigate to the home page
    await goto('/', { waitUntil: 'networkidle' })

    // Check if the title is correct
    await expect(page).toHaveTitle(/Home/)

    // Check if the AppHeader is present by looking for a navigation link
    await expect(page.getByRole('link', { name: /Produtos/i }).first()).toBeVisible()
    
    // Check if there is some main content rendered
    await expect(page.locator('main')).toBeVisible()
  })
})
