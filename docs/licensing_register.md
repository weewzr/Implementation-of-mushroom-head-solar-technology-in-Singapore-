# Data, Figure and External-Asset Licensing Register

**Purpose:** prevent public-repository publication from being mistaken for permission to redistribute third-party data, figures or other assets. This register records the current licensing state; unknown permission is treated as **not cleared for redistribution**.

| Asset / source | Role in project | Current repository content | Licence / permission status | Redistribution status | Required action |
|---|---|---|---|---|---|
| EMA Solar webpage and 2026 solar-target release | Singapore context and cited numerical statements | Bibliographic links and paraphrased factual context only | Public government webpages; specific reuse terms not yet recorded in this register | No copied dataset/figure presently redistributed | Retain citations; verify terms before reproducing substantial media/assets |
| SERIS real-time irradiance monitoring page | Monitoring-network capability/provenance | Bibliographic link and paraphrased metadata only | Public page accessible; no open bulk historical time-series licence established by current project evidence | **Historical raw measurements not cleared for public redistribution** | Obtain authorised access and written/applicable terms before acquisition/publication |
| Future SERIS/NSR historical station data | Preferred measured irradiance/weather input | Not acquired | Unknown until access route/terms are obtained | **Do not commit raw data** unless permission explicitly allows it | Store restricted raw inputs outside Git; record licence and permitted derivatives |
| data.gov.sg / NEA datasets | Possible supplementary meteorological inputs | No raw dataset currently committed | Dataset-specific terms must be checked; some catalogue datasets identify Singapore Open Data Licence | Dataset-specific; not globally pre-cleared | Record exact dataset/version/licence in acquisition manifest before use |
| NREL Solar Position Algorithm report / Reda & Andreas publications | Solar-position validation reference | Bibliographic metadata / citation only | Citation/reference use only in current repository | No third-party full text or figures redistributed | Continue citation; verify rights before reproducing figures/tables |
| Duffie & Beckman textbook | Solar-engineering reference | Bibliographic metadata / paraphrased equations/context only | Copyrighted book | **No scans/figures/pages for redistribution** | Cite; derive/explain independently rather than copying protected presentation |
| pvlib documentation | Secondary provenance for Cooper approximation | Bibliographic URL and attribution only | Public software documentation; exact documentation licence not recorded here | No copied documentation assets | Keep citation; verify licence if substantial documentation text/code is reused |
| Project conceptual SVG figures under `figures/` | Original explanatory visuals | Repository-generated project figures | Project-generated; no external asset dependency currently recorded | Cleared as project-created subject to confirming no embedded third-party assets | Keep source/edit history; audit embedded fonts/images if introduced |
| Model-generated plots / datasets | Future analytical/simulation outputs | Not yet substantive | Project-generated, but may inherit restrictions from input data | Publish only when upstream input terms permit derived-output publication | Record input provenance, code commit and derivative-publication permission |

## Rules

1. A public URL does not by itself grant permission to redistribute the underlying data or media.
2. Citation permission and raw-data redistribution permission are separate questions.
3. Unknown or ambiguous licence status defaults to **not cleared for redistribution**.
4. Restricted raw data must remain outside the public repository. Commit acquisition instructions, metadata and permitted derivatives instead.
5. Every newly acquired dataset must have a `data/` acquisition manifest recording provider, product/version, retrieval date, terms, redistribution status, variables, units, timestamps, preprocessing and source identifier/checksum when permitted.
6. Every non-project-created figure must record creator/source, licence, modification status and attribution requirement before inclusion.
7. Project-generated outputs derived from restricted inputs require a separate check that publication of aggregates/derivatives is permitted.
8. Licensing status must be rechecked before final public handover.

## Current status — 18 September 2026

**PARTIAL.** No known restricted raw SERIS dataset is committed. Current external-source use is primarily citation and paraphrased context. Exact dataset/asset licences remain to be recorded when specific data or third-party visual assets are selected.
