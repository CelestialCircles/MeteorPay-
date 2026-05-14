export type HeroMetric = {
  label: string;
  value: string;
  detail: string;
};

export type DashboardHighlight = {
  eyebrow: string;
  title: string;
  description: string;
};

export type SettlementStep = {
  tag: string;
  title: string;
  description: string;
};

export const heroMetrics: HeroMetric[] = [
  {
    label: "Average settlement",
    value: "2.4s",
    detail: "Designed around near-instant payment confirmation at the counter.",
  },
  {
    label: "Network fee profile",
    value: "<0.1%",
    detail: "Low-cost routing for merchant-friendly margins and repeat checkout.",
  },
  {
    label: "Asset coverage",
    value: "4 lanes",
    detail: "Stablecoins, native assets, and merchant-specific ecosystem tokens.",
  },
];

export const supportedAssets = ["XLM", "USDC", "EURC", "Loyalty Tokens"];

export const dashboardHighlights: DashboardHighlight[] = [
  {
    eyebrow: "Brand tokens",
    title: "Gradient-led design system",
    description:
      "MeteorPay now ships with reusable surfaces, premium color tokens, and a stronger typographic voice for the merchant suite.",
  },
  {
    eyebrow: "Dark mode",
    title: "Theme-aware by default",
    description:
      "A lightweight theme context keeps the new UI ready for both bright retail spaces and low-light operator workflows.",
  },
  {
    eyebrow: "Component base",
    title: "Reusable dashboard primitives",
    description:
      "Metric chips, feature cards, and shared buttons give future screens a consistent starting point instead of one-off styling.",
  },
  {
    eyebrow: "Project shape",
    title: "Structured for growth",
    description:
      "The `components`, `hooks`, `lib`, and `context` folders are in place for POS flows, wallet integrations, and analytics screens.",
  },
];

export const settlementFlow: SettlementStep[] = [
  {
    tag: "01",
    title: "Generate a merchant invoice",
    description:
      "Create a dynamic or static QR request with the amount, asset lane, and order context ready for wallet scan.",
  },
  {
    tag: "02",
    title: "Capture the customer signature",
    description:
      "The buyer signs from a compatible wallet while the POS keeps the merchant focused on the in-person handoff.",
  },
  {
    tag: "03",
    title: "Confirm and reconcile",
    description:
      "Indexer-backed status updates flow back into the dashboard so treasury, reports, and receipts stay aligned.",
  },
];
