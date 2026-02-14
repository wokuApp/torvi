import { Button as BaseButton } from '@base-ui/react/button'
import { type ComponentPropsWithoutRef } from 'react'

type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'outline'

const variantClasses: Record<ButtonVariant, string> = {
  primary:
    'bg-orange-500 text-white hover:bg-orange-600 active:bg-orange-700',
  secondary:
    'bg-gray-100 text-gray-900 hover:bg-gray-200 active:bg-gray-300',
  ghost:
    'bg-transparent text-gray-700 hover:bg-gray-100 active:bg-gray-200',
  outline:
    'border border-gray-300 text-gray-700 hover:bg-gray-50 active:bg-gray-100',
}

const baseClasses =
  'inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orange-500 disabled:opacity-50 disabled:cursor-not-allowed'

export interface ButtonProps extends ComponentPropsWithoutRef<'button'> {
  variant?: ButtonVariant
}

export function Button({
  variant = 'primary',
  className = '',
  children,
  ...props
}: ButtonProps) {
  return (
    <BaseButton
      className={`${baseClasses} ${variantClasses[variant]} ${className}`}
      {...props}
    >
      {children}
    </BaseButton>
  )
}
