import { FeatureCard } from "@/components/home/feature-card";
import { MetricChip } from "@/components/home/metric-chip";
import { ThemeToggle } from "@/components/theme-toggle";
import {
  dashboardHighlights,
  heroMetrics,
  settlementFlow,
  supportedAssets,
} from "@/lib/design-system";

export default function Home() {
  return (
    <main className="relative isolate overflow-hidden">
      <div className="meteor-grid pointer-events-none absolute inset-0 -z-20 opacity-70" />
      <div className="pointer-events-none absolute inset-x-0 top-0 -z-10 h-[44rem] bg-[radial-gradient(circle_at_top,rgba(255,255,255,0.12),transparent_64%)] dark:bg-[radial-gradient(circle_at_top,rgba(255,255,255,0.04),transparent_64%)]" />

      <section className="mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 py-8 lg:px-10">
        <header className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="flex h-11 w-11 items-center justify-center rounded-2xl bg-[linear-gradient(135deg,var(--meteor-cyan),var(--meteor-nova))] text-sm font-black uppercase tracking-[0.2em] text-meteor-midnight">
              MP
            </div>
            <div>
              <p className="font-[family:var(--font-display)] text-lg font-semibold tracking-[0.08em] text-meteor-ink uppercase">
                MeteorPay
              </p>
              <p className="text-sm text-copy-soft">
                Merchant settlements for digital assets
              </p>
            </div>
          </div>
          <ThemeToggle />
        </header>

        <div className="grid flex-1 gap-12 py-16 lg:grid-cols-[1.1fr_0.9fr] lg:items-center lg:py-20">
          <div className="max-w-3xl">
            <div className="eyebrow inline-flex items-center gap-2 rounded-full px-4 py-2 text-xs font-semibold uppercase tracking-[0.28em] text-copy-muted">
              <span className="h-2 w-2 rounded-full bg-meteor-cyan" />
              Instant QR settlements
            </div>
            <h1 className="mt-8 font-[family:var(--font-display)] text-5xl font-semibold leading-[1.02] tracking-[-0.04em] text-meteor-ink sm:text-6xl lg:text-7xl">
              Accept stablecoins with a{" "}
              <span className="text-gradient">premium merchant command center</span>.
            </h1>
            <p className="mt-6 max-w-2xl text-lg leading-8 text-copy-muted sm:text-xl">
              MeteorPay brings QR checkout, low-fee settlement, and treasury
              visibility into one polished surface for modern merchants who want
              crypto payments to feel as fast as a card tap.
            </p>

            <div className="mt-10 flex flex-col gap-4 sm:flex-row">
              <a
                href="#platform"
                className="button-primary inline-flex items-center justify-center rounded-full px-7 py-4 text-sm font-semibold tracking-[0.12em] text-meteor-midnight uppercase transition-transform duration-200 hover:-translate-y-0.5"
              >
                Explore the platform
              </a>
              <a
                href="#flow"
                className="button-secondary inline-flex items-center justify-center rounded-full px-7 py-4 text-sm font-semibold tracking-[0.12em] text-meteor-ink uppercase transition-colors duration-200 hover:bg-surface"
              >
                View settlement flow
              </a>
            </div>

            <div className="mt-12 grid gap-4 sm:grid-cols-3">
              {heroMetrics.map((metric) => (
                <MetricChip key={metric.label} {...metric} />
              ))}
            </div>

            <div className="mt-10 flex flex-wrap items-center gap-3 text-sm text-copy-soft">
              <span className="uppercase tracking-[0.22em]">Accepted assets</span>
              {supportedAssets.map((asset) => (
                <span
                  key={asset}
                  className="rounded-full border border-border bg-surface-alt px-4 py-2 text-copy-muted"
                >
                  {asset}
                </span>
              ))}
            </div>
          </div>

          <div className="glass-panel relative overflow-hidden rounded-[2rem] p-6 sm:p-8">
            <div className="absolute inset-x-8 top-0 h-px bg-[linear-gradient(90deg,transparent,rgba(25,213,221,0.5),transparent)]" />
            <div className="flex items-center justify-between gap-4">
              <div>
                <p className="text-sm uppercase tracking-[0.24em] text-copy-soft">
                  Merchant cockpit
                </p>
                <h2 className="mt-2 font-[family:var(--font-display)] text-2xl font-semibold text-meteor-ink">
                  Real-time revenue, fees, and checkout health
                </h2>
              </div>
              <div className="rounded-full border border-border bg-surface-alt px-4 py-2 text-xs font-semibold uppercase tracking-[0.22em] text-copy-muted">
                Live
              </div>
            </div>

            <div className="mt-8 grid gap-4 sm:grid-cols-2">
              <div className="rounded-[1.5rem] border border-border bg-surface-strong p-5">
                <p className="text-sm text-copy-soft">Today&apos;s volume</p>
                <p className="mt-3 text-4xl font-semibold tracking-[-0.04em] text-meteor-ink">
                  $18.4k
                </p>
                <p className="mt-2 text-sm text-copy-muted">
                  Up 18.2% from last trading day
                </p>
              </div>
              <div className="rounded-[1.5rem] border border-border bg-surface-alt p-5">
                <p className="text-sm text-copy-soft">Settlement speed</p>
                <p className="mt-3 text-4xl font-semibold tracking-[-0.04em] text-meteor-ink">
                  2.4s
                </p>
                <p className="mt-2 text-sm text-copy-muted">
                  Average merchant confirmation time
                </p>
              </div>
            </div>

            <div className="dashboard-rail relative mt-6 grid gap-4 pl-6">
              {settlementFlow.map((step) => (
                <div
                  key={step.title}
                  className="rounded-[1.5rem] border border-border bg-surface-alt p-5"
                >
                  <div className="flex items-center justify-between gap-3">
                    <p className="font-semibold text-meteor-ink">{step.title}</p>
                    <span className="rounded-full bg-[rgba(25,213,221,0.14)] px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.18em] text-meteor-cyan">
                      {step.tag}
                    </span>
                  </div>
                  <p className="mt-2 text-sm leading-6 text-copy-muted">
                    {step.description}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>

      <section id="platform" className="mx-auto w-full max-w-7xl px-6 pb-24 lg:px-10">
        <div className="mb-10 max-w-2xl">
          <p className="text-sm font-semibold uppercase tracking-[0.26em] text-copy-soft">
            Premium foundation
          </p>
          <h2 className="mt-4 font-[family:var(--font-display)] text-4xl font-semibold tracking-[-0.04em] text-meteor-ink">
            A frontend system shaped around fast payments and merchant trust.
          </h2>
          <p className="mt-4 text-lg leading-8 text-copy-muted">
            The first pass establishes a branded surface, dark-mode ready tokens,
            and reusable building blocks for the dashboard, POS, and reporting
            experiences mentioned in the project roadmap.
          </p>
        </div>

        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-4">
          {dashboardHighlights.map((feature) => (
            <FeatureCard key={feature.title} {...feature} />
          ))}
        </div>
      </section>

      <section id="flow" className="mx-auto w-full max-w-7xl px-6 pb-28 lg:px-10">
        <div className="glass-panel rounded-[2rem] p-8 sm:p-10">
          <div className="flex flex-col gap-6 lg:flex-row lg:items-end lg:justify-between">
            <div className="max-w-2xl">
              <p className="text-sm font-semibold uppercase tracking-[0.26em] text-copy-soft">
                Checkout choreography
              </p>
              <h2 className="mt-4 font-[family:var(--font-display)] text-4xl font-semibold tracking-[-0.04em] text-meteor-ink">
                Designed to make crypto checkout feel calm at the counter.
              </h2>
            </div>
            <p className="max-w-xl text-base leading-7 text-copy-muted">
              From QR generation to settlement visibility, each layer is framed
              around simplicity for merchants and near-instant feedback for
              customers.
            </p>
          </div>

          <div className="mt-10 grid gap-4 lg:grid-cols-3">
            {settlementFlow.map((step, index) => (
              <div
                key={`${step.title}-detail`}
                className="rounded-[1.75rem] border border-border bg-surface-alt p-6"
              >
                <p className="text-sm font-semibold uppercase tracking-[0.22em] text-copy-soft">
                  Step {index + 1}
                </p>
                <h3 className="mt-3 text-xl font-semibold text-meteor-ink">
                  {step.title}
                </h3>
                <p className="mt-3 text-sm leading-7 text-copy-muted">
                  {step.description}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>
    </main>
  );
}
