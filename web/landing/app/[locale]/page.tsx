import { Navbar } from '@/components/Navbar'
import { HeroSection } from '@/components/HeroSection'
import { HowItWorks } from '@/components/HowItWorks'
import { Features } from '@/components/Features'
import { BracketVisual } from '@/components/BracketVisual'
import { OpenSource } from '@/components/OpenSource'
import { Footer } from '@/components/Footer'

export default function Home() {
  return (
    <main>
      <Navbar />
      <HeroSection />
      <BracketVisual />
      <HowItWorks />
      <Features />
      <OpenSource />
      <Footer />
    </main>
  )
}
