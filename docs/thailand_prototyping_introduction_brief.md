# LaminarForge Thailand Prototyping Introduction Brief

Status: ready for advisor introduction; no outreach has been sent

Tickets: T-5293DD56, T-6C240EDA

Date: 2026-08-31

## The project

LaminarForge is developing open hardware for reproducible tissue-on-chip work.
The immediate goal is not to run cells, open a Thai company, or build a private
lab. It is to turn one small microfluidic design into a manufacturable no-cell
prototype, prove water/dye flow, and build a working relationship with a Thai
engineer and fabrication vendor.

## The first engagement

LaminarForge wants to buy a half-day or full-day session with an
English-speaking microfluidics or mechanical DFM engineer. Alex will own and
revise the design. The advisor should challenge it, redline it, and explain the
manufacturing tradeoffs in real time.

Requested outputs:

1. Review the proposed Rev D0 single-lane PMMA/clear-bottom flow coupon.
2. Identify geometry that cannot be machined, bonded, inspected, or tested.
3. Recommend a practical material, bond method, fittings, tolerances, and
   inspection method.
4. Return a marked-up STEP/PDF package and a short decision list.
5. Quote three bonded no-cell coupons plus one unbonded inspection witness.
6. If the advisor is not the fabricator, introduce one suitable Thai shop.

The controlled review/RFQ draft is in
`docs/microfluidic_chip_revd_first_article_review_rfq.md`.

## Deliberately excluded from the first article

- Cells, tissue, pathogens, viral vectors, or biological claims
- TEER electrodes, thin-film temperature sensors, or electronics
- Multi-organ routing, chip stacking, automation, or a 16-chip cassette
- Sterility, medical-device, or pressure-rating claims
- Production tooling or volume manufacturing

## What LaminarForge brings

- Parametric Rust CAD and the existing Rev D concept source
- A narrow proposed Rev D0 geometry for redlining
- A syringe-pump workstream for later no-cell flow testing
- Fast design iteration during the paid session
- Payment as a customer for consulting and prototype fabrication

## A successful first week in Thailand

- One paid DFM session completed
- One corrected two-part STEP and drawing package
- One written fabrication quote with lead time and inspection scope
- One selected bond method and fitting strategy
- Three no-cell coupons ordered only after the review blockers are closed

## Ninety-minute first meeting

| Time | Topic | Decision |
| --- | --- | --- |
| 0-15 min | Goal, use case, and exclusions | Keep the article water/dye only |
| 15-40 min | Channel, chamber, port, and bond review | Redline unmanufacturable geometry |
| 40-60 min | Material and process options | Select the cheapest credible route |
| 60-75 min | Tolerances and inspection | Freeze measurable acceptance criteria |
| 75-90 min | Deliverables, quote, and next meeting | Agree price, owner, and date |

## Draft introduction

```text
Subject: Paid microfluidic DFM review and prototype fabrication in Thailand

Hello,

I run LaminarForge, an open-hardware tissue-on-chip project. I will be in Thailand and am looking for an English-speaking engineer who can spend a paid half day or full day reviewing a small PMMA microfluidic design with me.

I will own and revise the CAD. I need practical DFM feedback on channel geometry, bonding, fittings, tolerances, inspection, and a quote for three simple water/dye-only prototypes. There are no cells, pathogens, or biological experiments in this first phase.

Can you provide the review directly, or introduce the right engineer or fabrication shop? I can send a short review packet before the meeting.

Thank you,
Alex Lewis
LaminarForge
```
