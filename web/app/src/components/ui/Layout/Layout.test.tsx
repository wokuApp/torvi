import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { LanguageProvider } from '@/i18n/LanguageContext'
import { Layout } from './Layout'

describe('Layout', () => {
  it('should render navbar and outlet content', async () => {
    const router = createMemoryRouter(
      [
        {
          element: <Layout />,
          children: [{ path: '/', element: <div>Page Content</div> }],
        },
      ],
      { initialEntries: ['/'] }
    )
    render(
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    )

    // Navbar logo
    expect(await screen.findByText('Torvi')).toBeInTheDocument()
    // Outlet content
    expect(screen.getByText('Page Content')).toBeInTheDocument()
  })

  it('should have main content area with max-width', async () => {
    const router = createMemoryRouter(
      [
        {
          element: <Layout />,
          children: [
            { path: '/', element: <div data-testid="content">Content</div> },
          ],
        },
      ],
      { initialEntries: ['/'] }
    )
    render(
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    )

    const content = await screen.findByTestId('content')
    const main = content.closest('main')
    expect(main).toHaveClass('max-w-5xl')
  })
})
